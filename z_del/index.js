(async () => {
  const NETWORK = "testnet";
  const CONTRACT_ID = "lamp.trying.testnet";
  const { connect, keyStores, WalletConnection, Contract } = nearApi;

  // ------------------------------------------
  const near = await connect(config());
  const wallet = new WalletConnection(near, `${NETWORK}-custom-prefix`); // supports logging in with different keys on the same domain
  const dom = setupDOMBindings();

  const contract = new Contract(wallet.account(), CONTRACT_ID, {
    viewMethods: ["get_all_lamps", "get_owner"],
    changeMethods: ["set_lamp_state", "disable_lamp", "enable_lamp"],
    sender: wallet.account(),
  });

  if (wallet.isSignedIn()) {
    const accountId = wallet.getAccountId();
    dom.btnConnect.innerHTML = `Disconnect ${accountId}`;
    dom.btnConnect.addEventListener("click", signOut);

    dom.btnHistory.addEventListener("click", history);
    dom.btnSubmit.addEventListener("click", submit);
  } else {
    dom.btnConnect.addEventListener("click", signIn);
  }

  async function history() {
    let out = await contract.get_all_lamps();
    let myJson = "";
    for (const item in out) {
      myJson +=
        out[item].lamp_id +
        "-" +
        out[item].lamp_state +
        "-" +
        out[item].lamp_type +
        "\n";
    }

    ouputText.innerHTML = myJson;
  }

  // ----------------
  // Helper functions
  // ----------------

  function signIn() {
    wallet.requestSignIn({
      // contractId: NETWORK === "testnet" ? "unv.testnet" : "unv.near",
      contractId: CONTRACT_ID,
      /* methodNames: ["get_all_lamps"], */
      viewMethods: ["get_all_lamps", "get_owner"],
      changeMethods: ["set_lamp_state", "disable_lamp", "enable_lamp"],
    });
  }

  function signOut() {
    wallet.signOut();
    btnConnect.innerHTML = "Login";
  }

  function config(network = NETWORK) {
    return {
      testnet: {
        networkId: "testnet",
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
      },
      mainnet: {
        networkId: "mainnet",
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: "https://rpc.mainnet.near.org",
        walletUrl: "https://wallet.mainnet.near.org",
        helperUrl: "https://helper.mainnet.near.org",
        explorerUrl: "https://explorer.mainnet.near.org",
      },
    }[network];
  }

  function setupDOMBindings() {
    return {
      btnConnect: document.querySelector("#connect"),
      btnHistory: document.querySelector("#history"),
      btnSubmit: document.querySelector("#submit"),
      ouputText: document.querySelector("#ouputText"),
    };
  }
})();
