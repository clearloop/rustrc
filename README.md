# Rustrc

![hero](./assets/hero-bg.jpg)

<br>
## Basquiat
<br>

#### 如何为结构体添加方法？

#### 如何改变结构体中的某个值？

#### 什么时候无法完成对已存在数据的复制？

#### 如何完整的复制一个结构体？浅拷贝，深拷贝。

#### 如何完成对与对象内部数据的复制？将引用来的结构体内部数据实例化。

#### mut 引用，传入以及用于引用的区别。

#### 是否要使用生命周期，传入引用与生命周期的区别。

#### 关于引用与传递所有权的利弊，优化内存的方法。

#### &str 与 std::string::String 的区别

#### 改变结构体内的值

#### String::from() 与 String.xxx 的区别, String::from 与 let string:String = "".to_string() 的区别，与 *.clone(), 深`Copy`，浅`Copy`

#### new defined ::: borrowed value has not live long enough??

#### String, Vec lifecircle, 

#### normal type can copy easily but complex, like "String", "Vec"..

#### 关于 `Copy`, 引用的区别，以及引用的各种情况

#### One time immutable, no more mutable.

#### 如无法实现 Copy 特性，是否能够通过闭包替代

#### closure `move` 特性

#### 借用 与 Copy 的取舍

<br>
## Andy 
<br>

#### &str && String

#### u8 && sclice && Vector && arr 

#### mut && Copy trait && Clone

#### lifecircle?

#### Iterator invalidation && use after release

#### String.into_bytes() 的长度远大于 BigInt_from_bytes(![x.into_bytes()]) []中?

#### BigInt_from(int > i32) 后转为字符串爆内存问题

#### What happended in PoW Shl compare, how to figure out a block is mined out.

#### Conver u8 to i32

#### BigInt

#### BigInt from i32 && BigInt from bytes?

#### b"1" == "1".tostring().intobytes() !

#### How to convert Sha256 hex to bytes, conver to bigint and then convert back it? here is the solution.

#### &Vec<u8> = &[u8], &String = &str --- 多态

#### What is the type of hash bytes? Bianary? Octal? Hex?

#### Slice and &[u8] and Vec<u8> and hex, Binary, ??

#### Rust 动态打印

#### [Rust slower](1)

## Schulz

#### What is Some() && Ok?

#### And Result

## Paul

#### Loop - iter() and enumrate().

#### 强制转换 int 类型问题

#### cannot move out of indexed content. When you use an index operator ([]) you get the actual object at index location. You do not get a reference, pointer or copy. 

#### out_idx 超出 i32 ?

#### About rust vector append, what is &Struct? 

#### mut is not only &COPY; even a TYPE;

#### How do I copy a String from a &String while iterating through a Vector?

#### What the fuck of to_owned??? really awesome!!

#### what the fuck of .to_owned and borrow?

#### &mut

#### figure loop loop errorrrrr

#### Options and Reuslt

#### What is UTXO? to verify one transaction, input first or output first... lol

We need to find all unspent transaction outputs (UTXO). Unspent means that these outputs weren’t referenced in any inputs. 


#### How to check if hashmap contains a key rust

#### Hashmap entry

#### How can I convert None Type. is some() - is_none()

#### how to counteract immutable borrow and mutable

important to\_owned()
```rust
let mut _trans = _unspent_outputs.get(&in_txid).unwrap().to_owned();
//??? _trans.append(&mut vec![(_out_idx as i32)]);
_unspent_outputs.remove(&in_txid);
```

#### Hashmap get return a refference?

#### loop name and 'static lifecycle?

#### How to iterate through a Hashmap, print the key/value and remove?

#### loop -> to_owned string & Vec || clone ????

#### hasher.result() to String -> `hex::encode(hasher.result())`

#### Loop and scope - let; change the define while loop? iterator.

without enumrate() directly.

#### about TXInput and TXOutput, 

```rust
type TXInput struct {
    tx_id: Vec<u8>, // hash id, one transfer.
    vout: Vec<u8>, // value out
    script_sig:  // from whom? 
}

type TXOutpt struct {
    value: i32,
    scriptpubkey: // to whom? 
}

```

## Vincent

Every transaction input in Bitcoin is signed by the one who created the transaction. Every transaction in Bitcoin must be verified before being put in a block. Verification means (besides other procedures):

1. Checking that inputs have permission to use outputs from previous transactions.
2. Checking that the transaction signature is correct.

#### RUST what for &.?

#### digest

#### Vec<u8>, generic array.

#### &mut and borrow, || 

#### secp256k1 -- invaild signature && invalid SecretKey

```rust
let signatrue = 
    [0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1];
```

#### why 1 first, ripemd160 && sha256. checksum...

#### 并不是动态的，而是叠加的一种方法，how many things can save time do?

#### hash_pubkey, version, checksum, address

Method: 
```
address = version + hash_pubkey + checksum;
version = self definedl
hash_pubkey = hash(pubkey);
checksum = hash(version + pubkey_hash);
```

```rust
pubkey_hash = [196, 220, 32, 216, 82, 18, 196, 122, 16, 158, 28, 75, 134, 199, 246, 127, 50, 124, 167, 167];

checksum = [75, 99, 204, 243, 218, 192, 67, 109, 192, 233, 22, 129, 5, 253, 118, 64, 49, 124, 220, 59, 182, 1, 124, 160, 102, 192, 126, 188, 19, 158, 255, 55];

address = [0, 196, 220, 32, 216, 82, 18, 196, 122, 16, 158, 28, 75, 134, 199, 246, 127, 50, 124, 167, 167, 75, 99, 204, 243, 218, 192, 67, 109, 192, 233, 22, 129, 5, 253, 118, 64, 49, 124, 220, 59, 182, 1, 124, 160, 102, 192, 126, 188, 19, 158, 255, 55];

```

test: 

```rust
let mut version = vec![00];
let mut pubkey_hash = hash_pubkey(self.pub_key.to_owned());
let mut _test = pubkey_hash.to_owned();
_test.remove(1);_test.resize(19,0);
println!("pubkey_hash: {:?}", &pubkey_hash);
version.append(&mut pubkey_hash);

let mut checksum = check_sum(version.to_owned());
version.append(&mut checksum);
println!("version: {:?}", &version);
println!("pubhash_key: {:?}", &_test);
let address = version.to_base58();
return address.into_bytes();
```

print:
```
pubkey_hash: [81, 181, 97, 115, 109, 51, 20, 80, 181, 169, 147, 240, 241, 26, 13, 22, 205, 131, 132, 65]
version: [0, 81, 181, 97, 115, 109, 51, 20, 80, 181, 169, 147, 240, 241, 26, 13, 22, 205, 131, 132, 65, 115, 168, 105, 177, 115, 215, 185, 22, 45, 53, 195, 219, 58, 6, 171, 73, 147, 111, 193, 78, 249, 112, 160, 175, 71, 221, 116, 214, 138, 56, 219, 27]
pubhash_key: [81, 97, 115, 109, 51, 20, 80, 181, 169, 147, 240, 241, 26, 13, 22, 205, 131, 132, 65]
```

[1]:https://www.reddit.com/r/rust/comments/7w3v77/why_is_my_rust_code_100x_slower_than_python/
