# ink!athon based (DE)centralized News Broadcasting System - DENBS

This is a full-stack dApp for ink! smart contracts with an integrated frontend. It is meant as POC for a decentralized news broadcasting system. While the frontend and some modules needs to be improved, the smart contract was the focus to give a raw idea.

[![asciicast](https://asciinema.org/a/fTaOQ1Inb8bVP2sloldTB54pS.svg)](https://asciinema.org/a/fTaOQ1Inb8bVP2sloldTB54pS)

---

**Table of Contents:**

1. [About 📖](#about-)
2. [Getting started 🚀](#getting-started-)
   1. [1. Run the frontend](#1-run-the-frontend)
   2. [2. Build \& deploy contracts on a local node](#2-build--deploy-contracts-on-a-local-node)
   3. [3. Connect the frontend to the local node](#3-connect-the-frontend-to-the-local-node)
3. [Customization 🎨](#customization-)
   1. [1. Project Name](#1-project-name)
   2. [2. Custom Contracts](#2-custom-contracts)
   3. [3. Custom Scripts](#3-custom-scripts)
4. [The Stack 🥞](#the-stack-)
5. [Deployment 🚢](#deployment-)
   1. [Environment Variables](#environment-variables)
   2. [Contract Deployment](#contract-deployment)
6. [VSCode Setup 🛠](#vscode-setup-)
   1. [Workspace](#workspace)
   2. [Plugins](#plugins)
7. [FAQs \& Troubleshooting 💬](#faqs--troubleshooting-)

---

## About 📖

Think AFP news hub, now think how you could expand on that when you have a decentralized network of news, notification, features articles with tokenomics, DEFI, roles to earn rewards in a non permissive way. This is what DENBS is about. It is a decentralized news broadcasting system that allows news hubs to share news with each other. The news hubs are incentivized to share news with each other and to verify the news they receive. The news hubs are also incentivized to share news with the public. The public can verify the news they receive from the news hubs.

DENBS is a decentralized news broadcasting system that aims to bridge frontiers and bring people together by breaking down the barriers of language and culture. With the help of generative AI, DENBS will be able to translate news from one language to another. This will allow people from different countries to read news from other countries in their own language. This will also allow people from different countries to share news with each other in their own language.

With a marketplace for advertising, the news hubs can earn money by selling advertizing space on their news. The news hubs can also earn money by selling news to other news hubs. The news hubs can also earn money by selling news to the public. The public can earn money by verifying news. The public can also earn money by sharing news with other people. Influencer can automatically share news with their followers and earn money from the advertizing space on their news and earn rewards for broadcasting as subscribers. Publishers and subscribers could also bid and ask in the marketplace for topics, slot spaces for content or ads.

NFT's as placeholder for news, news articles, notifications, alerts, events, channels will push the boundaries of what is possible.

Access to micro-bloggers, difficult to setup overseas social media platforms, messaging apps, you name it, everything automated, all the burden of translation, verification, and broadcasting is taken care of by the DENBS.

Have a source of data and broadcast it everywhere, in every language, in every country, in every culture, in every corner of the world. This is the power of DENBS.

Pitch Deck: https://pitch.com/public/ff5b2108-e86d-412f-ae11-1cad96c4a292

Investors Pitch Presentation 1: https://odysee.com/DENBS-Pitch:1

Hackathon Pitch Presentation 2: https://odysee.com/Hackaton-Presentation:c

### Use cases

While the use cases are endless, here are some examples:

1- Alice is a game developer and she wants to show her game to the world. She can use DENBS to publish her game ads campaign into other places with high visualization and conversion rates, She go to DENBS market place and place bids for NFT ads placeholder with high conversion rates. Once the deal is done and respecting deal parametrization and business rules her ads should be broadcasted and put into the right placeholders.

2- Bob is a news content creator, while his content is great, he is not a good marketer. He can use DENBS to offer his content to the world. Dave owns a digital news agency hub startup. While he is a good publisher, he is not a good content creator. He can use DENBS to offer his publishing services to the world. Bob and Dave can use DENBS to connect with each other and do business together. Dave wants to monetize his news hub by selling ads space. He can use DENBS to offer NFT ads placeholder to the world.

3- Charlie is an influencer, he has a lot of followers on social media. He can use DENBS to offer feature posts as NFT. Alice owns a DEX crypto startup. She wants to promote her DEX to the world. She can use DENBS to offer her DEX ads campaign to the world. Charlie and Alice can use DENBS to connect with each other and do business together.

4- Dave is a crypto trader and wants to sell buy and sell signals to his customers using a subscription plan. He can use DENBS to offer his buy and sell signals to the world. Eve owns a crypto robot trader. She wants to automate asset management with market signals. She can use DENBS to place bids for market signals. Dave and Eve can use DENBS to connect with each other and do business together.

5- Frank owns a local newspaper and is looking to increase content with international news. George owns an international news agency hub. Frank and George can use DENBS to connect with each other and do business together.

6- Henry created his own meme token and is looking to gain community traction. Isaac owns a crypto community group on telegram. Henry and Isaac can use DENBS to connect with each other and do business together.

7- Jack own a movie studio. Kate owns an on demand streaming service. Jack and Kate can use DENBS to connect with each other and do business together.

8- Larry is a social media user and follow Mary, a digital influencer selling feature posts on DENBS. He wants to monetize while reading posts. He can use DENBS to validate Mary posts by staking to become a validator. Larry is selected as validator for an Epoch provide feedback on Mary posts claiming rewards.

9- Mary is a crypto investor and wants to stake her assets to earn rewards. She can use DENBS to delegate her stake to Larry, a validator. Mary and Larry can use DENBS to connect with each other and do business together.

10- Nancy has an account in a social platform blocked to the rest of the world. Mary wants to reach Nancy followers. Mary can use DENBS to broadcast her posts to Nancy followers with automatic language translation.

11- Oscar and his friends are all readers of a newspaper used to buy content using DENBS. The newspaper is looking to filter content claimed as fake news. Oscar and his friends can use DENBS to validate content by staking to become a validator. Oscar is selected as validator for an Epoch provide feedback on content claiming rewards.

12- Peter owns a crypto metaverse. He can use DENBS to offer ads placeholder as NFT. Eve owns a business with a customer base on Peter's metaverse. She can use DENBS to broadcast her ads campaign to Peter's metaverse.

13- Quentin owns a smart contract and is looking for data outside the blockchain. Eve own an oracle node with curated data. Quentin and Eve can use DENBS to connect with each other and do business together.

14- Rachel owns a box with GPU. She can use DENBS to run a Broadcaster node client by staking and Becoming a broadcaster. She is selected as broadcaster for an Epoch and broadcast content to subscribers earning rewards.

15- Sarah owns a smartphone and want to monetized it by staking and running a broadcaster node on her phone. Charlie owns and account with twitter and want to replicate his posts to other social media platforms. Sarah and Charlie can use DENBS to connect with each other and do business together.

### 1. Run the frontend

The frontend works out of the box, without a local node running, as the sample contract is pre-deployed on certain live testnets (i.e. `alephzero-testnet` and `shibuya`). Necessary deployment metadata and addresses are provided under `contracts/deployments/`.

> **Pre-requisites:**
>
> - Setup Node.js v16+ (recommended via [nvm](https://github.com/nvm-sh/nvm))
> - Install [pnpm](https://pnpm.io/installation) (recommended via [Node.js Corepack](https://nodejs.org/api/corepack.html))
> - Clone this repository

```bash
# Install dependencies (once)
# NOTE: This automatically creates an `.env.local` file
pnpm install

# Start Next.js frontend
pnpm run dev
```

Optionally, to enable [`simple-git-hooks`](https://github.com/toplenboren/simple-git-hooks) (for automatic linting & formatting when committing), you can run the following command once: `pnpm simple-git-hooks`.

NOTE: At this point we don't have a proper frontend yet, so you can skip this step.
To have a glimpse of the work in progress, you can run the following command once: `pnpm run sandbox`. Or try the scripts in the contracts folder.

### 2. Build & deploy contracts on a local node

The `contracts/package.json` file contains shorthand scripts for building, testing, and deploying your contracts. To run these scripts, you need to set `contracts/` as the active working directory in your terminal.

> **Pre-requisites:**
>
> - Install Rust via the [Substrate Docs](https://docs.substrate.io/install/) (skip the "Compile a Substrate node" section)
> - Install [`cargo contract`](https://github.com/paritytech/cargo-contract)
> - Install [`substrate-contracts-node`](https://github.com/paritytech/substrate-contracts-node)

```bash
# Build contracts and move artifacts to `contracts/deployments/{contract}/` folders
pnpm run build

# Start local node with persistence (contracts stay deployed after restart)
# NOTE: When using Brave, shields have to be taken down for the UIs
pnpm run node

## IMPORTANT: Open a separate terminal window and keep the node running

# Deploy the contracts on the local node
pnpm run deploy
```

Alternatively, you can also deploy contracts manually using [Contracts UI](https://contracts-ui.substrate.io/) (`pnpm contracts-ui`) in the browser.

### 3. Connect the frontend to the local node

Open the `frontend/.env.local` file and set the `NEXT_PUBLIC_DEFAULT_CHAIN` variable to `development`. Then restart the frontend and you should be able to interact with the contracts deployed on your local node.

_Read more about environment variables and all available chain constants in the [Environment Variables](#environment-variables) section below._

## Customization 🎨

### 1. Architecture

We have used Azero.id router as a means to configure the resolution of our smart contracts by name leveraging the get_registry function in the ink! smart contract. This allows us to have a single smart contract that can be used to resolve all the other smart contracts. This is a very powerful feature of ink! smart contracts coupled with Azero.id.

```rust
        pub fn router_resolve_contract(&self, name: String) -> Option<AccountId> {
            use azns_integration::contract_ref::{AznsRouter, AznsRouterRef};
            let router = AznsRouterRef::from(self.domain_router);
            router.get_registry(name)
        }

```

The broker node client, yet to be created, will be used to connect to publisher source of data and broadcast it to the network by lookup on onchain data of subscribers to topics.

Every epoch a set of subscribers will be selected to broadcast and another set to verify the news they receive. We are looking to allow flexible business models based on budget, subscription, and other parameters.

The broker node client will also be used to connect to the news hubs and verify the news they receive. The broker node client will also be used to connect to the public and verify the news they receive.

### 2. Custom Contracts

So far we have created the following contracts:

- `Epoch` – A simple contract that allows you to define the epoch period in block and optional offset.
- `Registry` – A simple contract that allows publishers to register their news hub as topics.
- `TopicMeta` – A simple contract that allows publishers to associate generic data to their topics as a means to define data source setup and retrieval.
- `Subscription` – A simple contract that allows subscribers to bind to topics as broadcasters by staking.
- `Delegation` – A simple contract that allows investors to delegate stake to subscribers.
- `Dispute` – A simple contract that allows publishers and subscribers to resolve conflicts by the help of judges.
- `Router` – A simple contract that mocks the Azero.id router for development purposes.

They are all located in the `contracts/` directory and have a README with more information about their purpose and usage.

### 3. Custom Scripts

We have created the following scripts:

- `sandbox` – A simple script that allows you to run like the frontend in a sandbox mode. Currently it showcases the workflow for `Registry`, `TopicMeta` contract.
- `deployAll` – A simple script that setup the contracts and their dependencies.
- `subscriberbox` – A simple script that showcases the subscriber workflow.

## The Stack 🥞

<details>
<summary><strong>The Stack in Detail</strong></summary>

- Monorepo Workspace with `contracts/` and `frontend/` directories as packages.
- Package Manager: `pnpm` or `yarn@stable` (Read more in the [FAQs](#faqs--troubleshooting) section below)
- Smart Contract Development: Rust, ink!, `cargo-contract`, `substrate-contracts-node`
- Frontend: Next.js, React, TypeScript
  - Contract Interactions: `polkadot-js`, [`useInkathon`](https://github.com/scio-labs/use-inkathon) React Hooks & Utility Library (alternatively: [`useInk`](https://use.ink/frontend/getting-started))
  - Styling: `chakra`, `tailwindcss`, `twin.macro`, `emotion`
  - Linting & Formatting: `eslint`, `prettier`, `simple-git-hooks`, `lint-staged`

<small>Styling, linting, and formatting libraries can be fully dropped or replaced with alternatives.</small>

</details>

![inkathon Stack Diagram](inkathon-stack-diagram.png)

> [!NOTE]  
> When opening the project directory in VSCode, it automatically suggests opening the `inkathon.code-workspace` file instead. This is recommended as it offers a more predictable monorepo configuration.

## Deployment 🚢

Spinning up a deployment via Vercel is pretty straightforward as the necessary settings are already configured in `vercel.json`. If you haven't cloned the repository yet, you can also use the **Deploy** button below to create a new repository from this template.

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https%3A%2F%2Fgithub.com%2Fvercel%2Fnext.js%2Ftree%2Fcanary%2Fexamples%2Fhello-world&env=NEXT_PUBLIC_DEFAULT_CHAIN&envDescription=Insert%20%60alephzero-testnet%60%20or%20%60shibuya%60&envLink=https%3A%2F%2Fgithub.com%2Fscio-labs%2Finkathon%23environment-variables&project-name=inkathon&repository-name=inkathon&redirect-url=https%3A%2F%2Fgithub.com%2Fscio-labs%2Finkathon&demo-url=https%3A%2F%2Finkathon.xyz)

### Environment Variables

One key element making this boilerplate so flexible is the usage of environment variables to configure the active network in the frontend. This is done by setting the `NEXT_PUBLIC_DEFAULT_CHAIN` variable in the `frontend/.env.local` file, or in the Vercel deployment settings respectively.

<details>
<summary><strong>All Supported Chain Constants</strong></summary>

| Network Identifier  | Name                    | Type    |
| ------------------- | ----------------------- | ------- |
| `development`       | ️Local Development Node | Testnet |
| `alephzero-testnet` | Aleph Zero Testnet      | Testnet |
| `rococo`            | Rococo                  | Testnet |
| `shibuya`           | Shibuya Testnet         | Testnet |
| `shiden`            | Shiden                  | Mainnet |
| `alephzero`         | Aleph Zero              | Mainnet |
| `astar`             | Astar                   | Mainnet |

<small>Source: https://github.com/scio-labs/use-inkathon/blob/main/src/chains.ts</small>

> [!NOTE]  
> Chains can also be supplied manually by creating a [`SubstrateChain`](https://github.com/scio-labs/use-inkathon/blob/main/src/chains.ts#L4) object. If you think a chain is missing, please open an issue or PR.

</details>

All environment variables are imported from `process.env` in [`frontend/src/config/environment.ts`](https://github.com/scio-labs/inkathon/blob/main/frontend/src/config/environment.ts) for type safety.

| Environment Variables           | [Default Values](https://github.com/scio-labs/inkathon/blob/main/frontend/.env.local.example) | Description                                                                                                                                                         |
| ------------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `NEXT_PUBLIC_DEFAULT_CHAIN` \*️⃣ | ️`alephzero-testnet`                                                                          | The network (Substrate-based chain) the frontend should connect to by default and what contract deployment artifacts to import.                                     |
| `NEXT_PUBLIC_PRODUCTION_MODE`   | `false`                                                                                       | Optional boolean flag to differentiate production environment (e.g. for SEO or Analytics).                                                                          |
| `NEXT_PUBLIC_URL`               | `http://localhost:3000`                                                                       | Optional string that defines the base URL of the frontend (will be auto-inferred from Vercel environment variables).                                                |
| `NEXT_PUBLIC_SUPPORTED_CHAINS`  | –                                                                                             | Optional array with network identifers (e.g. `["alephzero-testnet", "shibuya"]`) that are supported by the frontend, **if the dApp is supposed to be multi-chain**. |

<small>\*️⃣ Required </small>

### Contract Deployment

In the [Getting Started](#getting-started) section above, we've already deployed the sample `Greeter` contract on a local node. To target a live network, we can use the `CHAIN` environment variable when running the `deploy` script.

```bash
CHAIN=alephzero-testnet pnpm run deploy
```

Further, dynamically loaded environment files with the `.env.{chain}` naming convention can be used to add additional configuration about the deployer account.

```bash
# .env.alephzero-testnet
ACCOUNT_URI=bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice
```

When running the same script again, this deployer account defined there will be used to sign the extrinsic.

> [!WARNING]  
> These files are gitignored by default, but you should still be extra cautious when adding sensitive information to them.

## VSCode Setup 🛠

### Workspace

It's recommended to develop in VSCode by opening the workspace file `inkathon.code-workspace` instead of just the plain directory. This approach offers multiple advantages, including a more predictable monorepo configuration. VSCode will also automatically suggest switching to the workspace when opening the project's root directory in the bottom right corner.

<img src="inkathon-vscode-workspace.png" width="400" alt="VSCode Workspace Notification">

### Plugins

Additionally, the VSCode plugins listed below are recommended as they can be very helpful when working with this boilerplate.

<details>
<summary><strong>All Recommended Plugins</strong></summary>

| Plugin Name                                                                                                                            | Description                                  |
| -------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| [`dbaeumer.vscode-eslint`](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)                                 | Adds ESLint editor support.                  |
| [`esbenp.prettier-vscode`](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)                                 | Adds Prettier editor support.                |
| [`bradlc.vscode-tailwindcss`](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)                           | Adds tailwindcss editor support.             |
| [`lightyen.tailwindcss-intellisense-twin`](https://marketplace.visualstudio.com/items?itemName=lightyen.tailwindcss-intellisense-twin) | Adds twin.macro editor support.              |
| [`rust-lang.rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)                               | Adds Rust language support.                  |
| [`ink-analyzer.ink-analyzer`](https://marketplace.visualstudio.com/items?itemName=ink-analyzer.ink-analyzer)                           | Adds ink! language support.                  |
| [`tamasfe.even-better-toml`](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)                             | Adds `.toml` file support.                   |
| [`gruntfuggly.todo-tree`](https://marketplace.visualstudio.com/items?itemName=gruntfuggly.todo-tree)                                   | Lists all `TODO` comments in your workspace. |
| [`wayou.vscode-todo-highlight`](https://marketplace.visualstudio.com/items?itemName=wayou.vscode-todo-highlight)                       | Lists all `TODO` comments in your workspace. |
| [`mikestead.dotenv`](https://marketplace.visualstudio.com/items?itemName=mikestead.dotenv)                                             | Adds syntax highlighting for `.env` files.   |

</details>

## FAQs & Troubleshooting 💬

<details>
<summary><strong>Which package managers are supported? Do I have to use pnpm?</strong></summary>

For monorepo workspaces, [pnpm](https://pnpm.io) is likely the fastest and most reliable choice. When using it though, it's strongly recommended everyone on the team uses it. No installs should be performed nor any other lock files should be committed.

As an alternative, [yarn](https://yarnpkg.com/) is also supported and can be used for installation. Caveats when using yarn:

- Only the stable version of yarn (currently v3) is supported, not [yarn classic](https://classic.yarnpkg.com/) (v1).
- `yarn.lock` files should be committed instead of `.pnpm-lock.yaml` files.
- The `pnpm` CLI is still used in many `package.json` scripts, so these would have to be adjusted manually.

> [!IMPORTANT]  
> As [npm](https://www.npmjs.com/) lacks support for the `workspace` import protocol, it's not compatible with ink!athon.

</details>

<details>
<summary><strong>How to solve `Cannot find module './greeter/development.ts'`?</strong></summary>

Sometimes, Next.js doesn't pick up changes (i.e. file creations) in the `contracts/deployments/{contract}/` folders correctly. E.g., when you just deployed on a local node for the first time and set the frontend's `.env.local` to connect to the `development` network.

To fix this, you can delete the build cache at `frontend/.next`. This is currently the only solution and will force Next.js to rebuild the project and pick up the new files.

> [!NOTE]  
> To prevent this behavior, the `contracts/package.json` file contains a small `postinstall` script that creates an empty `development.ts` file if none exists.

</details>

<details>
<summary><strong>How to approach styling?</strong></summary>

This boilerplate currently offers styling via the following options.

- [Chakra UI](https://chakra-ui.com/) – Component library for quick prototyping e.g. during hackathons)
- [twin.macro](https://github.com/ben-rogerson/twin.macro) – [Tailwindcss](https://tailwindcss.com/) within Styled Components via [Emotion](https://emotion.sh/docs/styled) (see [snippets](#snippets))
- Standard (S)CSS styles via `className` and `*.module.(s)css` files.

> [!IMPORTANT]  
> To reduce the bundle size in production, it's recommended to use either option 1 or 2, but not both.

</details>

<details>
<summary><strong>Can I just use plain TailwindCSS?</strong></summary>

The packages mentioned above can be replaced with vanilla TailwindCSS manually without much effort.

> [!NOTE]  
> We are currently transitioning from twin.macro to vanilla TailwindCSS as the new default. This will be reflected in the boilerplate soon.

</details>

<details>
<summary><strong>Resources to learn more about Substrate, ink!, and polkadot.js</strong></summary>

- [ink! Documentation](https://use.ink/)
- [polkadot.js Documentation](https://polkadot.js.org/docs/)
- [Polkadot Wiki ink! Tools](https://wiki.polkadot.network/docs/build-open-source)
- [Aleph Zero Documentation](https://docs.alephzero.org/aleph-zero/build/)
- [ink!athon Workshop Recording](https://youtube.com/watch?v=SoNLZfsd0mQ)
- [ink!athon Telegram Group](https://t.me/inkathon)

</details>
