(async () => {
  const NETWORK = "testnet";
  const CONTRACT_ID = "lamp.trying.testnet";
  const { connect, keyStores, WalletConnection, Contract } = nearApi;

  // ------------------------------------------
  const near = await connect(config());
  const wallet = new WalletConnection(near, `${NETWORK}-custom-prefix`); // supports logging in with different keys on the same domain
  const dom = setupDOMBindings();

  const contract = new Contract(wallet.account(), CONTRACT_ID, {
    viewMethods: ["get_all_lamps", "get_lamp", "get_owner"],
    changeMethods: ["set_lamp_state", "disable_lamp", "enable_lamp"],
    sender: wallet.account(),
  });

  if (wallet.isSignedIn()) {
    const accountId = wallet.getAccountId();
    dom.btnConnect.innerHTML = `Disconnect ${accountId}`;
    dom.btnConnect.addEventListener("click", signOut);

    dom.btnHistory.addEventListener("click", history);
    dom.btnSubmit.addEventListener("click", submit);
    dom.contractName.innerHTML = `Connected to : ${contract.contractId}`;
    populateLamp();
    // dom.ddSelectLamp.addEventListener("change", populateLamp);
  } else {
    dom.btnConnect.addEventListener("click", signIn);
    dom.contractName.innerHTML = `Connected to : None`;
  }

  async function history() {
    let ddSelectedIndex = dom.ddSelectLamp.selectedIndex;
    let ddSelLampStateValue = dom.ddSelectLamp[ddSelectedIndex].value;
    let ddSelectedValue = dom.ddSelectLamp[ddSelectedIndex].textContent;
    //let ddSelection = dd.options[value.selectedIndex].value;
    let lampDetail = await contract.get_lamp({
      lamp_id: parseInt(ddSelectedValue),
    });
    let data = JSON.stringify(lampDetail);
    dom.ouputText.innerHTML = data;
  }

  async function populateLamp() {
    let lampList = await contract.get_all_lamps();
    dom.ddSelectLamp.innerHTML = "";
    for (const lamp in lampList) {
      var dd = document.createElement("option");
      dd.textContent = lamp;
      dd.value = lampList[lamp].lamp_state;
      dom.ddSelectLamp.appendChild(dd);
    }
  }

  // ----------------
  // Helper functions
  // ----------------

  function signIn() {
    wallet.requestSignIn({
      // contractId: NETWORK === "testnet" ? "unv.testnet" : "unv.near",
      contractId: CONTRACT_ID,
      viewMethods: ["get_all_lamps", "get_owner"],
      changeMethods: ["set_lamp_state", "disable_lamp", "enable_lamp"],
    });
  }

  function signOut() {
    wallet.signOut();
    dom.btnConnect.innerHTML = "Login";
    dom.contractName.innerHTML = `Connected to : None`;
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
      contractName: document.querySelector("#contractName"),
      ddSelectLamp: document.querySelector("#selectLamp"),
    };
  }
})();
