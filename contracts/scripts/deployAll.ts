import { ContractPromise } from '@polkadot/api-contract'
import { contractQuery, contractTx, decodeOutput, deployContract } from '@scio-labs/use-inkathon'
import * as dotenv from 'dotenv'
import { getDeploymentData } from './utils/getDeploymentData'
import { initPolkadotJs } from './utils/initPolkadotJs'
import { writeContractAddresses } from './utils/writeContractAddresses'

// [KEEP THIS] Dynamically load environment from`.env.{chainId}`
const chainId = process.env.CHAIN || 'development'
dotenv.config({ path: `.env.${chainId}` })

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
  const { api, chain, account } = await initPolkadotJs(chainId, accountUri)

  const gasLimit = 100000n * 1000000n
  const pay_zero = 0n
  const storageDepositLimit = null

  // Deploy router contract
  const { abi: routerAbi, wasm: routerWasm } = await getDeploymentData('router')
  const router = await deployContract(api, account, routerAbi, routerWasm, 'new', [])
  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    router,
  })

  const router_contract = new ContractPromise(api, routerAbi, router.address)

  const { abi: epochAbi, wasm: epochWasm } = await getDeploymentData('epoch')
  const epoch_offset = api.createType('BlockNumber', 0)
  const epoch_period = api.createType('BlockNumber', 10)
  const epoch = await deployContract(api, account, epochAbi, epochWasm, 'new', [
    epoch_offset,
    epoch_period,
  ])
  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    epoch,
  })

  const epoch_address = api.createType('AccountId', epoch.address)
  const epoch_string = api.createType('String', 'epoch')

  if (chainId == 'development') {
    // insert epoch
    try {
      await contractTx(api, account, router_contract, 'insert', {}, ['epoch', epoch_address])
      console.log('\nSuccessfully insert epoch')

      // Read epoch
      const result = await contractQuery(
        api,
        account.address,
        router_contract,
        'aznsRouter::getRegistry',
        {},
        [epoch_string],
      )
      const { decodedOutput } = decodeOutput(result, router_contract, 'aznsRouter::getRegistry')
      console.log('\nQueried epoch:', decodedOutput)
    } catch (error) {
      console.error('Error while insert epoch', error)
    }
  }

  const { abi: registryAbi, wasm: registryWasm } = await getDeploymentData('registry')
  const domain_router = api.createType('AccountId', router.address)
  const registry = await deployContract(api, account, registryAbi, registryWasm, 'new', [
    domain_router,
  ])

  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    registry,
  })

  const registry_address = api.createType('AccountId', registry.address)

  if (chainId == 'development') {
    // insert registry
    try {
      await contractTx(api, account, router_contract, 'insert', {}, ['registry', registry_address])
      console.log('\nSuccessfully insert registry')

      // Read registry
      const result = await contractQuery(
        api,
        account.address,
        router_contract,
        'aznsRouter::getRegistry',
        {},
        ['registry'],
      )
      const { decodedOutput } = decodeOutput(result, router_contract, 'aznsRouter::getRegistry')
      console.log('\nQueried registry:', decodedOutput)
    } catch (error) {
      console.error('Error while insert registry', error)
    }
  }

  const { abi: subscriptionAbi, wasm: subscriptionWasm } = await getDeploymentData('subscription')
  const subscription = await deployContract(
    api,
    account,
    subscriptionAbi,
    subscriptionWasm,
    'new',
    [domain_router],
  )

  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    subscription,
  })

  const subscription_address = api.createType('AccountId', subscription.address)

  if (chainId == 'development') {
    // insert subscription
    try {
      await contractTx(api, account, router_contract, 'insert', {}, [
        'subscription',
        subscription_address,
      ])
      console.log('\nSuccessfully insert subscription')

      // Read subscription
      const result = await contractQuery(
        api,
        account.address,
        router_contract,
        'aznsRouter::getRegistry',
        {},
        ['subscription'],
      )
      const { decodedOutput } = decodeOutput(result, router_contract, 'aznsRouter::getRegistry')
      console.log('\nQueried subscription:', decodedOutput)
    } catch (error) {
      console.error('Error while insert subscription', error)
    }
  }

  const { abi: delegationAbi, wasm: delegationWasm } = await getDeploymentData('delegation')
  const delegation = await deployContract(api, account, delegationAbi, delegationWasm, 'new', [
    domain_router,
  ])

  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    delegation,
  })

  const delegation_address = api.createType('AccountId', delegation.address)

  if (chainId == 'development') {
    // insert delegation
    try {
      await contractTx(api, account, router_contract, 'insert', {}, [
        'delegation',
        delegation_address,
      ])
      console.log('\nSuccessfully insert delegation')

      // Read delegation
      const result = await contractQuery(
        api,
        account.address,
        router_contract,
        'aznsRouter::getRegistry',
        {},
        ['delegation'],
      )
      const { decodedOutput } = decodeOutput(result, router_contract, 'aznsRouter::getRegistry')
      console.log('\nQueried delegation:', decodedOutput)
    } catch (error) {
      console.error('Error while insert delegation', error)
    }
  }

  const { abi: topicmetaAbi, wasm: topicmetaWasm } = await getDeploymentData('topicmeta')
  const topicmeta = await deployContract(api, account, topicmetaAbi, topicmetaWasm, 'new', [
    domain_router,
  ])

  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    topicmeta,
  })

  const topicmeta_address = api.createType('AccountId', topicmeta.address)

  if (chainId == 'development') {
    // insert topicmeta
    try {
      await contractTx(api, account, router_contract, 'insert', {}, [
        'topicmeta',
        topicmeta_address,
      ])
      console.log('\nSuccessfully insert topicmeta')

      // Read topicmeta
      const result = await contractQuery(
        api,
        account.address,
        router_contract,
        'aznsRouter::getRegistry',
        {},
        ['topicmeta'],
      )
      const { decodedOutput } = decodeOutput(result, router_contract, 'aznsRouter::getRegistry')
      console.log('\nQueried topicmeta:', decodedOutput)
    } catch (error) {
      console.error('Error while insert topicmeta', error)
    }
  }

  const { abi: disputeAbi, wasm: disputeWasm } = await getDeploymentData('dispute')
  const judger = api.createType('AccountId', account.address)
  const dispute = await deployContract(api, account, disputeAbi, disputeWasm, 'new', [
    judger,
    domain_router,
  ])

  // Write contract addresses to `{contract}/{network}.ts` file(s)
  await writeContractAddresses(chain.network, {
    dispute,
  })

  const dispute_address = api.createType('AccountId', dispute.address)

  if (chainId == 'development') {
    // insert dispute
    try {
      await contractTx(api, account, router_contract, 'insert', {}, ['dispute', dispute_address])
      console.log('\nSuccessfully insert dispute')

      // Read dispute
      const result = await contractQuery(
        api,
        account.address,
        router_contract,
        'aznsRouter::getRegistry',
        {},
        ['dispute'],
      )
      const { decodedOutput } = decodeOutput(result, router_contract, 'aznsRouter::getRegistry')
      console.log('\nQueried dispute:', decodedOutput)
    } catch (error) {
      console.error('Error while insert dispute', error)
    }
  }
}

main()
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
  .finally(() => process.exit(0))
