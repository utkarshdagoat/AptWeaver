import { ethers, Contract, BigNumberish } from "ethers";
import { bytesToHex } from '@ethereumjs/util';
import { FeeMarketEIP1559Transaction, FeeMarketEIP1559TxData } from '@ethereumjs/tx';
import { Common } from '@ethereumjs/common';
import { keccak256, sha3_256 } from "js-sha3";
import { ec as EC } from "elliptic";
import bs58 from "bs58";
import readline from "readline"
function genrateRootPk() {
  const x = "66a8e29452ee4166046da0b18891b9b2366d56d3b2234eca799ff48ace9c7371".padStart(64, '0');
  const y = "b7c30aefedeea99bffab1ffbdf705d7775e5aa5e9ed2ff32baad213546811922".padStart(64, '0');
  const uncompressedPublicKey = x + y;
  const base58Encoded = bs58.encode(Buffer.from(uncompressedPublicKey, 'hex'));
  return `secp256k1:${base58Encoded}`;
}

function baseDecode(value: string) {
  return new Uint8Array(bs58.decode(value))
}

export function najPublicKeyStrToUncompressedHexPoint() {
  const rootPublicKey = genrateRootPk()
  console.log("base58", rootPublicKey.split(':')[0])
  const res = '04' + Buffer.from(baseDecode(rootPublicKey.split(':')[1])).toString('hex');
  return res;
}
function convert(hex: string) {
  if (hex.length % 2) { hex = '0' + hex; }
  const bn = BigInt('0x' + hex);
  return bn
}
export async function deriveChildPublicKeyAptos(
  parentUncompressedPublicKeyHex: string,
  signerId: string,
  path = ''
) {
  const ec = new EC('secp256k1');
  console.log(parentUncompressedPublicKeyHex)
  // Generate a scalar using sha3_256 with signerId and path
  const scalarHex = sha3_256(`aptos-mpc-recovery v0.1.0 epsilon derivation:${signerId},${path}`);
  console.log("scaler bn without div", convert(scalarHex))
  const scalerBn = BigInt(Math.floor(Number(convert(scalarHex)) / (6)));
  console.log("scaler bn", scalerBn)

  // Extract x and y coordinates from the parent public key
  const x = parentUncompressedPublicKeyHex.substring(2, 66);
  const y = parentUncompressedPublicKeyHex.substring(66);

  console.log("x", x)
  console.log("y", y)

  // Create the old public key point
  const oldPublicKeyPoint = ec.curve.point(x, y);
  console.log("scaler hex", scalarHex)
  // Multiply the scalar by the curve generator
  const scalarTimesG = ec.g.mul(scalarHex);

  // Add the result to the old public key point to get the new public key point
  const newPublicKeyPoint = oldPublicKeyPoint.add(scalarTimesG);

  const newX = newPublicKeyPoint.getX().toString('hex').padStart(64, '0');
  const newY = newPublicKeyPoint.getY().toString('hex').padStart(64, '0');

  console.log("sum", "04" + newX + newY)
  return '04' + newX + newY;  // Uncompressed secp256k1 public key
}

export function uncompressedHexPointToEvmAddress(uncompressedHexPoint: string) {
  const addressHash = keccak256(`0x${uncompressedHexPoint.slice(2)}`);
  console.log("addessHash", addressHash)

  // Ethereum address is last 20 bytes of hash (40 characters), prefixed with 0x
  return ("0x" + addressHash.substring(addressHash.length - 40));
}
interface GasPrice {
  maxFeePerGas: bigint;
}

export class Ethereum {
  provider: ethers.providers.JsonRpcProvider;
  chain_id: number;

  constructor(chain_rpc: string, chain_id: number) {
    this.provider = new ethers.providers.JsonRpcProvider(chain_rpc);
    this.chain_id = chain_id;
    this.queryGasPrice();
  }

  async deriveAddress(accountId: string, derivation_path: string): Promise<{ publicKey: Buffer; address: string }> {
    const publicKey = await deriveChildPublicKeyAptos(najPublicKeyStrToUncompressedHexPoint(), accountId, derivation_path);
    const address = uncompressedHexPointToEvmAddress(publicKey);
    return { publicKey: Buffer.from(publicKey, 'hex'), address };
  }

  async queryGasPrice(): Promise<GasPrice> {
    const maxFeePerGas = await this.provider.getGasPrice();
    const maxPriorityFeePerGas = await this.provider.send("eth_maxPriorityFeePerGas", []);
    return { maxFeePerGas: maxFeePerGas.toBigInt() };
  }

  async getBalance(accountId: string): Promise<string> {
    const balance = await this.provider.getBalance(accountId);
    return ethers.utils.formatEther(balance);
  }

  async getContractViewFunction(receiver: string, abi: any[], methodName: string, args: any[] = []): Promise<any> {
    const contract = new Contract(receiver, abi, this.provider);
    return await contract[methodName](...args);
  }

  createTransactionData(receiver: string, abi: any[], methodName: string, args: any[] = []): string {
    const contract = new Contract(receiver, abi);
    return contract.interface.encodeFunctionData(methodName, args);
  }

  async createPayload(sender: string, receiver: string, amount: string): Promise<{ transaction: FeeMarketEIP1559Transaction; payload: Uint8Array }> {
    const common = new Common({ chain: this.chain_id });

    // Get the nonce & gas price
    const nonce = await this.provider.getTransactionCount(sender);
    const { maxFeePerGas } = await this.queryGasPrice();
    console.log(nonce)
    // Construct transaction data
    const transactionData: FeeMarketEIP1559TxData = {
      nonce: nonce + 1,
      gasLimit: ethers.BigNumber.from(100000).toBigInt(),
      maxFeePerGas,
      maxPriorityFeePerGas: maxFeePerGas,
      to: receiver as `0x${string}`,
      value: ethers.utils.parseEther(amount).toBigInt(),
      chainId: this.chain_id,
    };

    // Create the EIP-1559 transaction
    const transaction = FeeMarketEIP1559Transaction.fromTxData(transactionData, { common });
    const payload = transaction.getHashedMessageToSign();

    return { transaction, payload };
  }

  async reconstructSignature(r: string, s: string, recovery_id: number, transaction: FeeMarketEIP1559Transaction): Promise<FeeMarketEIP1559Transaction> {
    // Reconstruct signature using r, s, and v (recovery_id)
    const rBuffer = Buffer.from(r, 'hex');
    const sBuffer = Buffer.from(s, 'hex');
    const v = recovery_id;

    const signedTransaction = transaction.addSignature(BigInt(v), rBuffer, sBuffer);

    if (signedTransaction.getValidationErrors().length > 0) {
      throw new Error("Transaction validation errors");
    }
    if (!signedTransaction.verifySignature()) {
      throw new Error("Signature is not valid");
    }
    return signedTransaction;
  }

  async relayTransaction(signedTransaction: FeeMarketEIP1559Transaction): Promise<string> {
    const serializedTx = bytesToHex(signedTransaction.serialize());
    const relayed = await this.provider.sendTransaction(serializedTx);
    return relayed.hash;
  }
}


