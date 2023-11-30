import { ethers, BigNumber } from "ethers";
import { isArray, parseInt } from "lodash";
import { useState } from "react";
import { atom, useRecoilState } from "recoil";

import { createContext } from "react";

import { getEnvironmentName } from "@/lib/app-helper";
import { readJSONFile } from "@/lib/file-helper";

const POLLING_INTERVAL = 12000;
//const RPC_URL = process.env.VUE_APP_RPC_URL;

import axios from "axios";
import { makeObservable, observable, action, computed , reaction} from "mobx";

const ENV_MODE = getEnvironmentName();
import serverConfig from "@/config/server-config.json";
 

async function walletIsUnlocked() {
  /* if (window.ethereum) {
    try {
      const accounts = await window.ethereum.request({
        method: "eth_accounts",
      });
      // return accounts;
      return true;
    } catch (error) {
      console.error("Error fetching accounts", error);
      //return [];
      return false;
    }
  } else {
    // return [];
    return false;
  }*/
  return await window.ethereum._metamask.isUnlocked();
}

const localServerConfig = serverConfig[ENV_MODE];
export class Web3Store {
  provider = undefined;
  signer = undefined;
  account = undefined;

  //  balance=0
  chainId = undefined;

  transactionCount = undefined;

  challenge = undefined;
  authToken = undefined;
  authTokenExpiresAt = undefined;
  authTokenScopes = [];
 

  /*   authorized = () => {

      return this.authToken !== undefined && this.authTokenExpiresAt > Date.now()
    }*/

  customCallbacks = {};

  constructor() {
    makeObservable(this, {
      provider: observable,
      signer: observable,
      account: observable,

      chainId: observable,

      //    balance: observable,
      challenge: observable,
      authToken: observable,
      authTokenExpiresAt: observable,
      authTokenScopes: observable,
 

      active: computed,
      authorized: computed,
      testnetMode: computed,

      loadState: action,
      connect: action,
      soft_reconnect: action,
      disconnect: action,
      registerWalletCallbacks: action,

      requestChallengeAndSign: action,

      registerCustomCallback: action,
    });
 
  }
  get active() {
    return this.account !== undefined;
  }

  get testnetMode() {
    console.log("chain id is ", this.chainId);
    return BigNumber.from(this.chainId ?? 0).eq(BigNumber.from("11155111"));
  }

  get authorized() {
    let isAuthed =
      this.authToken !== undefined &&
      this.authTokenExpiresAt !== undefined &&
      new Date(this.authTokenExpiresAt) > Date.now();

    console.log(
      { isAuthed },
      this.authTokenExpiresAt,
      new Date(this.authTokenExpiresAt) > Date.now()
    );

    return isAuthed;
  }

  async connect() {
    if (!window.ethereum) {
      //redirect to metamask url
      window.open("https://www.metamask.io", "_blank");

      console.log("You must install metamask ");

      return;
    }

    this.provider = new ethers.providers.Web3Provider(window.ethereum, "any");
    // Prompt user for account connections
    await this.provider.send("eth_requestAccounts", []);
    this.signer = this.provider.getSigner();
    console.log("Account:", await this.signer.getAddress());
    let account = await this.signer.getAddress();

    //  let balance = await this.signer.getBalance()
    //let balanceFormatted = ethers.utils.formatEther(balance)

    const { chainId } = await this.provider.getNetwork();

    this.account = account;
    // this.balance = balance
    // this.active = true
    this.chainId = BigNumber.from(chainId ?? 0).toString();

    console.log("set chain id ", this.chainId);

    //  this.registerWalletCallbacks();

    this.saveState();
  }

  async soft_reconnect() {
    this.provider = new ethers.providers.Web3Provider(window.ethereum, "any");
    // Prompt user for account connections
    // await this.provider.send("eth_requestAccounts", []);
    // this.signer = this.provider.getSigner();
    // console.log("Account:", await this.signer.getAddress());
    // let account = await this.signer.getAddress();

    //  let balance = await this.signer.getBalance()
    //let balanceFormatted = ethers.utils.formatEther(balance)

    const { chainId } = await this.provider.getNetwork();

    // this.account = account;
    // this.balance = balance
    // this.active = true

    this.chainId = BigNumber.from(chainId ?? 0).toString();

    // console.log("set chain id ", this.chainId);

    this.registerWalletCallbacks();

    //this.saveState();
  }

  async disconnect() {
    this.account = undefined;
    //  this.active = false
    this.balance = 0;

    this.authToken = undefined;

    this.saveState();
  }

  //these dont work properly like this w strict mode ...
  registerWalletCallbacks() {
    console.log("register wallet callbacks ", window.ethereum.isConnected());

    window.ethereum.on("connect", ({ chainId }) => {
      this.chainId = BigNumber.from(chainId ?? 0).toString();
      this.emitCustomEvent("connect");
    });

    window.ethereum.on("chainChanged", (chainId) => {
      this.chainId = BigNumber.from(chainId ?? 0).toString();
      this.emitCustomEvent("chainChanged");
      console.log("chain changed");
    });

    window.ethereum.on("accountsChanged", async (accounts) => {
      this.account = accounts[0];
      this.emitCustomEvent("accountsChanged");
      console.log("account changed");
    });
  }

  emitCustomEvent(name) {
    if (isArray(this.customCallbacks[name])) {
      for (let cb of this.customCallbacks[name]) {
        cb();
      }
    }
  }

  /*
      Allows for registering callbacks to trigger
      when certain wallet callbacks trigger such as accountsChanged
    */
  registerCustomCallback(name, callback) {
    if (!isArray(this.customCallbacks[name])) {
      this.customCallbacks[name] = [];
    }

    this.customCallbacks[name].push(callback);

    console.log("registered callback ", name);
  }

  async requestChallengeAndSign() {
    //request the challenge from the server
    // pop up metamask to personal sign it
    // submit that signature to the server

    //https://docs.metamask.io/wallet/reference/provider-api/
    const is_unlocked = await walletIsUnlocked();
    console.log("is unlocked", is_unlocked);
    if (!is_unlocked) {
      await this.connect();
    }

    const backendServerUrl = localServerConfig.backendServerUrl;

    let generateChallengeEndpointUrl = `${backendServerUrl}/api/session/generate_challenge`;

    let challengePostRequest = await axios.post(generateChallengeEndpointUrl, {
      public_address: this.account,
    });

    console.log({ challengePostRequest });

    if (challengePostRequest.status == 200) {
      let challenge = challengePostRequest.data.challenge;

      this.challenge = challenge;

      const publicAddress = this.account;

      const provider = new ethers.providers.Web3Provider(
        window.ethereum,
        "any"
      );

      let signature = await provider
        .getSigner(publicAddress)
        .signMessage(challenge);

      let generateSessionEndpointUrl = `${backendServerUrl}/api/session/generate_user_session`;

      let authorizationPostRequest = await axios.post(
        generateSessionEndpointUrl,
        {
          public_address: this.account,
          signature: signature,
          challenge: challenge,
        }
      );

      let { auth_token, expires_at, scopes } = authorizationPostRequest.data;

      this.authToken = auth_token;
      this.authTokenExpiresAt = expires_at * 1000; //what we get from server is seconds unix so we convert to ms
      this.authTokenScopes = scopes;

      console.log("set auth token", this.authToken, this.authTokenExpiresAt);

      this.saveState();

      return true;
    } else {
      console.error(challengeResponse.error);
    }

    return false;
  }

  saveState() {
    const state = {
      // Include the properties you want to save in localStorage
      authToken: this.authToken,
      authTokenExpiresAt: this.authTokenExpiresAt,
      authTokenScopes: this.authTokenScopes,
      account: this.account,
    };

    localStorage.setItem("w3Store", JSON.stringify(state));
  }

  // Load state from localStorage
  loadState() {
    const storedState = localStorage.getItem("w3Store");
    if (storedState) {
      const state = JSON.parse(storedState);
      // Update the store properties with the loaded state

      //if the loaded state is too old DONT load it, delete it?
      if (new Date(this.authTokenExpiresAt) > Date.now()) {
        console.log("tried to load expired state");
      } else {
        this.account = state.account;
        this.authToken = state.authToken;
        this.authTokenScopes = state.authTokenScopes;
        this.authTokenExpiresAt = state.authTokenExpiresAt;
      }
    }
  }
}

export async function requestAddNetwork({ chainId, chainName, rpcUrl }) {
  console.log("request add network");

  const params = [
    {
      chainId,
      chainName,
      rpcUrls: [rpcUrl],
      nativeCurrency: {
        name: "ETH",
        symbol: "ETH",
        decimals: 18,
      },
    },
  ];

  console.log({ params });
  let addedNetwork = await window.ethereum.request({
    id: 1,
    jsonrpc: "2.0",
    method: "wallet_addEthereumChain",
    params,
  });

  console.log({ addedNetwork });
}
 





export function getNetworkNameFromChainId(chainId) {
  switch (chainId) {
    case 1:
      return "mainnet";
    case 11155111:
      return "sepolia";
    case 5:
      return "goerli";
    //case 17001: return 'mainnet'
    //case 17005: return 'goerli'

    default:
      return "unknown";
  }
}

const web3Store = new Web3Store();

export const Web3StoreContext = createContext(web3Store);
