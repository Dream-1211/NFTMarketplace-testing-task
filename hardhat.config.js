require("@nomiclabs/hardhat-waffle");

const fs = require('fs')
const privateKey = fs.readFileSync(".secret").toString();
const projectId = "3218541e3d3441acbc2090f640263689";

module.exports = {
  networks: {
    hardhat: {
      chainId: 1337
    }
  },
  mumbai: {
    url: `https://polygon-mumbai.infura.io/v3/${projectId}`,
    account: [privateKey]
  },
  mainnet: {
    url: `https://polygon-mainnet.infura.io/v3/${projectId}`,
    account: [privateKey]
  },
  solidity: {
    version: "0.8.4",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
};

