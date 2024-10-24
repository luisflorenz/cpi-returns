Anchor is a Rust framework that facilitates the creation of dApps on the Solana blockchain. One of the peculiarities of developing on Solana is the need to use a lot of template code, which can be a rather time-consuming and costly process. Anchor makes this process much easier and reduces development time by providing various tools such as (de)serialization of accounts and instruction data.

Please refer to the official documentation at https://www.anchor-lang.com/docs/installation for instructions before proceeding to the next stage.

Let's start by initializing the project 👇:


  anchor init cpi-returns
  
Next, go to the project and build it to check that your environment is set up correctly


cd cpi-returns
anchor build

in the Cargo.lock file, find the "solana-program" package and change it to the previously recommended version and delete the "checksum" field completely. Repeat the command 


  anchor build
  
To be sure the environment is correct, call the command 👉:


  anchor test to test the project