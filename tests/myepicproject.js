const anchor = require('@project-serum/anchor');
const {SystemProgram} = anchor.web3;
const main = async () => {
  console.log("starting test...");

  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Myepicproject;

  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("transaction signature", tx);
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF COUNT', account.totalGifs.toString());
  await program.rpc.addGif("insert_a_giphy_likn_here", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF COUNT', account.totalGifs.toString());

  console.log("gift list", account.gifList);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.log(error);
    process.exit(1);
  }
}

runMain();