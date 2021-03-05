const Dogs = artifacts.require("Dogs");
const Proxy = artifacts.require("Proxy");

module.exports = async function (deployer, network, accounts) {
  const dogs = await Dogs.new();
  const proxy = await Proxy.new(dogs.address);

  var proxyDogs = await Dogs.at(proxy.address);
  await proxyDogs.setNumberOfDogs(10);
  var nrOfDogs = await proxyDogs.getNumberOfDogs();
  console.log(nrOfDogs.toNumber());
};
