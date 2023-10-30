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

  //   const gasLimit = 100000n * 1000000n
  //   const pay_zero = 0n
  //   const storageDepositLimit = null

  // Deploy router contract
  //   const { abi: routerAbi, wasm: routerWasm } = await getDeploymentData('router')

  const deployments = await getDeployments()

  //   const greeter = deployments.find((d) => d.contractId === ContractIds.Greeter)

  //   const contract_greeter = new ContractPromise(api, greeter.abi, greeter.address)

  //   // Update message
  //   try {
  //     await contractTx(api, account, contract_greeter, 'set_message', {}, ['Hello, script!'])
  //     console.log('\nSuccessfully updated greeting')
  //   } catch (error) {
  //     console.error('Error while updating greeting', error)
  //   }

  // Read message
  //   const result = await contractQuery(api, '', contract_greeter, 'greet')
  //   const { decodedOutput } = decodeOutput(result, contract_greeter, 'greet')
  //   console.log('\nQueried greeting:', decodedOutput)

  const epoch = deployments.find((d) => d.contractId === ContractIds.Epoch)
  const epoch_contract = new ContractPromise(api, epoch.abi, epoch.address)

  try {
    // Read get_offset
    const result = await contractQuery(api, account.address, epoch_contract, 'offset::get_offset')
    const { decodedOutput } = decodeOutput(result, epoch_contract, 'offset::get_offset')
    console.log('\nQueried get_offset:', decodedOutput)
  } catch (error) {
    console.error('Error while in epoch', error)
  }
  try {
    // Read get_offset
    const result = await contractQuery(
      api,
      account.address,
      epoch_contract,
      'period::get_period_length',
    )
    const { decodedOutput } = decodeOutput(result, epoch_contract, 'period::get_period_length')
    console.log('\nQueried get_period_length:', decodedOutput)
  } catch (error) {
    console.error('Error while in epoch', error)
  }

  const registry = await deployments.find((d) => d.contractId === ContractIds.Registry)
  const registry_contract = new ContractPromise(api, registry.abi, registry.address)
  const registry_name = api.createType('String', 'my.broadcast.topic')

  try {
    // Read valid
    const result = await contractQuery(api, account.address, registry_contract, 'valid', {}, [
      registry_name,
    ])
    const { decodedOutput } = decodeOutput(result, registry_contract, 'valid')
    console.log('\nQueried valid:', decodedOutput)
  } catch (error) {
    console.error('Error while in valid', error)
  }

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
  } catch (error) {
    console.error('Error while in get_Hash', error)
  }

  try {
    // Read rent_price
    const duration = api.createType('u32', 4)
    const result = await contractQuery(api, account.address, registry_contract, 'rentPrice', {}, [
      registry_name,
      duration,
    ])
    const { decodedOutput } = decodeOutput(result, registry_contract, 'rentPrice')
    console.log('\nQueried rentPrice:', decodedOutput)
  } catch (error) {
    console.error('Error while in rentPrice', error)
  }

  try {
    // query make_Commitment
    const secret = api.createType('u32', 0)
    const result = await contractQuery(
      api,
      account.address,
      registry_contract,
      'makeCommitment',
      {},
      [registry_name, account.address, secret],
    )
    const { decodedOutput } = decodeOutput(result, registry_contract, 'makeCommitment')
    console.log('\n query make_Commitment:', decodedOutput)

    // send commit
    try {
      const commitment = api.createType('Hash', decodedOutput)
      const payment = 100n
      const options = { value: payment, storageDepositLimit: null, gasLimit: -1 }
      await contractTx(api, account, registry_contract, 'commit', options, [commitment])
      console.log('\nSuccessfully send commit')

      // send register
      try {
        const duration = api.createType('u32', 4)

        const payment = 100000n
        const options = { value: payment, storageDepositLimit: null, gasLimit: -1, nonce: -1 }
        await contractTx(api, account, registry_contract, 'register', options, [
          registry_name,
          account.address,
          duration,
          secret,
        ])
        console.log('\nSuccessfully send register')

        try {
          // Read available
          const result = await contractQuery(
            api,
            account.address,
            registry_contract,
            'available',
            {},
            [registry_name],
          )
          const { decodedOutput } = decodeOutput(result, registry_contract, 'available')
          console.log('\nQueried available:', decodedOutput)
        } catch (error) {
          console.error('Error while in available', error)
        }
      } catch (error) {
        console.error('Error while in register', error)
      }
    } catch (error) {
      console.error('Error while in commit', error)
    }
  } catch (error) {
    console.error('Error while in makeCommitment', error)
  }

  const topicmeta = deployments.find((d) => d.contractId === ContractIds.TopicMeta)
  const topicmeta_contract = new ContractPromise(api, topicmeta.abi, topicmeta.address)

  try {
    // Read get_Hash
    const result = await contractQuery(api, account.address, registry_contract, 'getHash', {}, [
      registry_name,
    ])
    const { decodedOutput } = decodeOutput(result, registry_contract, 'getHash')
    console.log('\nQueried get_Hash:', decodedOutput)
    const hash = api.createType('Hash', decodedOutput)

    try {
      // Read topic meta link
      const result = await contractQuery(api, account.address, topicmeta_contract, 'get_link', {}, [
        hash,
      ])
      const { decodedOutput } = decodeOutput(result, topicmeta_contract, 'get_link')
      console.log('\nQueried topic meta link:', decodedOutput)
    } catch (error) {
      console.error('Error while topic meta link', error)
    }

    // send set link
    try {
      const options = { storageDepositLimit: null, gasLimit: -1, nonce: -1 }
      await contractTx(api, account, topicmeta_contract, 'set_link', options, [
        hash,
        'https://docs.my.broadcast.topic.com',
      ])
      console.log('\nSuccessfully set link')

      try {
        // Read topic meta link
        const result = await contractQuery(
          api,
          account.address,
          topicmeta_contract,
          'get_link',
          {},
          [hash],
        )
        const { decodedOutput } = decodeOutput(result, topicmeta_contract, 'get_link')
        console.log('\nQueried topic meta link:', decodedOutput)
      } catch (error) {
        console.error('Error while topic meta link', error)
      }
    } catch (error) {
      console.error('Error while set link', error)
    }

    try {
      // Read topic meta data
      const result = await contractQuery(
        api,
        account.address,
        topicmeta_contract,
        'get_capabilities',
        {},
        [hash],
      )
      const { decodedOutput } = decodeOutput(result, topicmeta_contract, 'get_capabilities')
      console.log('\nQueried topic get_capabilities:', decodedOutput)
    } catch (error) {
      console.error('Error while topic get_capabilities', error)
    }

    try {
      // Read get_Hash
      const result = await contractQuery(api, account.address, registry_contract, 'getHash', {}, [
        'platform',
      ])
      const { decodedOutput } = decodeOutput(result, registry_contract, 'getHash')
      console.log('\nQueried get_Hash:', decodedOutput)
      const platform_hash = api.createType('Hash', decodedOutput)

      // send set capability meta properties
      try {
        const options = { storageDepositLimit: null, gasLimit: -1, nonce: -1 }
        await contractTx(api, account, topicmeta_contract, 'set_capability', options, [
          hash,
          platform_hash,
          'twitter',
        ])
        console.log('\nSuccessfully set capability')

        try {
          // Read topic meta data
          const result = await contractQuery(
            api,
            account.address,
            topicmeta_contract,
            'get_capabilities',
            {},
            [hash],
          )
          const { decodedOutput } = decodeOutput(result, topicmeta_contract, 'get_capabilities')
          console.log('\nQueried topic get_capabilities:', decodedOutput)
        } catch (error) {
          console.error('Error while topic get_capabilities', error)
        }
      } catch (error) {
        console.error('Error while set link', error)
      }
    } catch (error) {
      console.error('Error while get_Hash', error)
    }
  } catch (error) {
    console.error('Error while Read get_Hash', error)
  }
}

main()
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
  .finally(() => process.exit(0))
