const Dogs = artifacts.require("Dogs");
const DogsUpdated = artifacts.require("DogsUpdated");
const Proxy = artifacts.require("Proxy");

module.exports = async function (deployer, network, accounts) {
  //deploy contracts
  const dogs = await Dogs.new();
  const proxy = await Proxy.new(dogs.address);

  //create proxy dog to fool truffle
  var proxyDogs = await Dogs.at(proxy.address);

  //set the number of dogs
  await proxyDogs.setNumberOfDogs(10);

  //testted
  var nrOfDogs = await proxyDogs.getNumberOfDogs();
  console.log("Before updating: " + nrOfDogs.toNumber());

  //update
  const dogsUpdated = await DogsUpdated.new();
  proxy.upgrade(dogsUpdated.address);

  //fool truffle once again. it now thinks proxyDogs has all functions
  proxyDogs = await DogsUpdated.at(proxy.address);
  //initialize proxy state.
  proxyDogs.initialize(accounts[0]);

  //check so that storage remained
  nrOfDogs = await proxyDogs.getNumberOfDogs();
  console.log("After update: " + nrOfDogs.toNumber());

  //set the number of dogs through the proxy with new func contract
  await proxyDogs.setNumberOfDogs(30);

  //check so that setNumberOfDogs worked with new func contract.
  nrOfDogs = await proxyDogs.getNumberOfDogs();
  console.log("After change: " + nrOfDogs.toNumber());
};
