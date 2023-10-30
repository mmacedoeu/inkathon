import { Keyring } from '@polkadot/api'
import { ContractPromise } from '@polkadot/api-contract'
import {
  SubstrateDeployment,
  contractQuery,
  contractTx,
  decodeOutput,
} from '@scio-labs/use-inkathon'
import * as dotenv from 'dotenv'
import { initPolkadotJs } from './utils/initPolkadotJs'

// [KEEP THIS] Dynamically load environment from`.env.{chainId}`
const chainId = process.env.CHAIN || 'development'
dotenv.config({ path: `.env.${chainId}` })

enum ContractIds {
  Greeter = 'greeter',
  Epoch = 'epoch',
  Router = 'router',
  Registry = 'registry',
  Subscription = 'subscription',
  TopicMeta = 'topicmeta',
  Delegation = 'delegation',
  Dispute = 'dispute',
}

const getDeployments = async (): Promise<SubstrateDeployment[]> => {
  const networks = [chainId]
  const deployments: SubstrateDeployment[] = []

  for (const networkId of networks) {
    for (const contractId of Object.values(ContractIds)) {
      let abi = await import(`../../contracts/deployments/${contractId}/${contractId}.json`, {
        assert: { type: 'json' },
      })
      abi = abi.default
      const { address } = await import(`../../contracts/deployments/${contractId}/${networkId}.ts`)

      deployments.push({ contractId, networkId, abi, address })
    }
  }

  return deployments
}

/**
 * Example script that updates & reads a message from a greeter contract.
 * Can be used as a template for other scripts.
 *
 * Parameters:
 *  - `DIR`: Directory to read contract build artifacts (optional, defaults to `./deployments`)
 *  - `CHAIN`: Chain ID (optional, defaults to `development`)
 *
 * Example usage:
 *  - `pnpm run script <script-name>`
 *  - `CHAIN=alephzero-testnet pnpm run script <script-name>`
 */
const main = async () => {
  // [KEEP THIS] Initialization
  const accountUri = process.env.ACCOUNT_URI || '//Alice'
  const { api, account } = await initPolkadotJs(chainId, accountUri)

  const keyring = new Keyring({ type: 'sr25519' })
  const dave = keyring.addFromUri('//Dave')

  const deployments = await getDeployments()

  const registry = await deployments.find((d) => d.contractId === ContractIds.Registry)
  const registry_contract = new ContractPromise(api, registry.abi, registry.address)
  const registry_name = api.createType('String', 'my.broadcast.topic')

  const subscription = deployments.find((d) => d.contractId === ContractIds.Subscription)
  const subscription_contract = new ContractPromise(api, subscription.abi, subscription.address)

  try {
    // Read available
    const result = await contractQuery(api, account.address, registry_contract, 'available', {}, [
      registry_name,
    ])
    const { decodedOutput } = decodeOutput(result, registry_contract, 'available')
    console.log('\nQueried available:', decodedOutput)
  } catch (error) {
    console.error('Error while in available', error)
  }

  try {
    // Read get_Hash
    const result = await contractQuery(api, account.address, registry_contract, 'getHash', {}, [
      registry_name,
    ])
    const { decodedOutput } = decodeOutput(result, registry_contract, 'getHash')
    console.log('\nQueried get_Hash:', decodedOutput)
    const hash = api.createType('Hash', decodedOutput)
    const subscriber_address = api.createType('AccountId', dave.address)

    try {
      // Read subscription
      const result = await contractQuery(
        api,
        dave.address,
        subscription_contract,
        'subscriptions::getSubscription',
        {},
        [hash, subscriber_address],
      )
      const { decodedOutput } = decodeOutput(
        result,
        subscription_contract,
        'subscriptions::getSubscription',
      )
      console.log('\nQueried subscription:', decodedOutput)
    } catch (error) {
      console.error('Error while Read subscription', error)
    }

    try {
      // Send subscription
      const value = 30n * 100n
      const options = { value, storageDepositLimit: null, gasLimit: -1 }

      await contractTx(api, account, subscription_contract, 'subscriptions::subscribe', options, [
        hash,
        subscriber_address,
      ])
      console.log('\nSuccessfully send commit')
    } catch (error) {
      console.error('Error while send subscription', error)
    }

    try {
      // Read subscription
      const result = await contractQuery(
        api,
        dave.address,
        subscription_contract,
        'subscriptions::getSubscription',
        {},
        [hash, subscriber_address],
      )
      const { decodedOutput } = decodeOutput(
        result,
        subscription_contract,
        'subscriptions::getSubscription',
      )
      console.log('\nQueried subscription:', decodedOutput)
    } catch (error) {
      console.error('Error while Read subscription', error)
    }
  } catch (error) {
    console.error('Error while in get_Hash', error)
  }
}

main()
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
  .finally(() => process.exit(0))
