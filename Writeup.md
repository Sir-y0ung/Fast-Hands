Encryptor

dificulty: easy
----------------
So challenge give us an elf file called encryptor and an encrypted elf

![challfiles](https://github.com/YoungFlexerGR/challDev/assets/82509480/f845edea-42fb-4b4c-a9cb-e871380fff49)


Quick start
------------
Runing strings is easy to detect that is a rust binary
![rsproof](https://github.com/YoungFlexerGR/challDev/assets/82509480/ff3ac120-efd2-4585-8804-51760aa05098)

Without wasting time we open it in ida!

Analyzing binary
------------------------
So loading up main we can see an interesting function called encrypt. Analyzing a bit
more we can understand that main actually parse the filename from command line arguments 
and encrypt it.

```rust
  v33 = 0;
  p_bytes = v13;
  encryptor::encrypt::h23b411f3d7ff7d1f(&v22, &p_bytes);
  v4 = _$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$::deref::hb7bffd1cb81b9047(&v22);
  v34 = 0;
  bytes = v11;
```


Jumping into encrypt function seems to be 2 stages of encryption

```rust
   v2 = _$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$::deref::hb7bffd1cb81b9047(p_bytes);
  alloc::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::to_vec::hd718b80b286be983(retstr, v2);
  v15 = 1;
  v8.length = (usize)rand::rngs::thread::thread_rng::hf54719191417eca1();
  *(core::ops::range::RangeInclusive<u8> *)&v3 = core::ops::range::RangeInclusive$LT$Idx$GT$::new::h6a5462bd24cb8201(
                                                   0xAu,
                                                   0x63u);
  v6 = v3;
  v16 = v3;
  v17 = BYTE2(v3);
  v10 = BYTE2(v3);
  v9 = v6;
  v4 = (BYTE2(v6) << 16) | (unsigned __int16)v6;
  v18 = rand::rng::Rng::gen_range::ha42b6ae1c09a27db(
          (rand::rngs::thread::ThreadRng *)&v8.length,
          (core::ops::range::RangeInclusive<u8>)v4);
  v15 = 0;
  p_byte_array = *retstr;
  encryptor::stage1::hfe9dbf8099d2b49f(&v11, &p_byte_array, v18);
  *retstr = v11;
  v15 = 0;
  v14 = *retstr;
  encryptor::stage2::hd312546730607046(&v13, &v14);
  *retstr = v13;
  v15 = 0;
  core::ptr::drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$::hdaad59d6b546f6dc((rand::rngs::thread::ThreadRng *)&v8.length);
  v15 = 0;
  core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$::hb00b364ba1638373(p_bytes);
  return retstr;
}
```

stage1
---------
Stage 1 gets 2 parameters, the bytes readed from the file and a completely random key.
Seems that iterate all bytes and pass them in this fuction with the random value. Searching a bit more seems that its a simple implementation of xor operation.
As result, all bytes are xored with a random value. 
```
return ~(randomval & byte) & (randomval | byte);
```

stage2
---------------
Going on stage 2, seems a bit more messy. There is a loop that iterates all xored bytes and shuffles 3 bytes each time 
![sh](https://github.com/YoungFlexerGR/challDev/assets/82509480/0b9a29e5-d5ea-4fbd-9b8c-9f98de48e427)

To sum up shuffle function get 3 bytes each time and moves the last byte at the beggining 
(for example 0x122334 -> 0x341223) 
```rust
 v5 = input.length;
  v17 = input.data_ptr;
  v18 = input.length;
  self = 4LL;
  index.start = 6LL;
  x.data_ptr = core::str::traits::_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$str$GT$::index::h2a087c6aaad94111(
                 input.data_ptr,
                 __PAIR128__(4LL, input.length),
                 __PAIR128__(&off_9ACB8, 6LL));
  x.length = input.length;
  v11 = 0LL;
  v12 = 4LL;
  index.end = core::str::traits::_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$str$GT$::index::h2a087c6aaad94111(
                input.data_ptr,
                v5,
                __PAIR128__(&off_9ACD0, 4LL));
  v10 = v2;
  v6 = core::fmt::ArgumentV1::new_display::hea555bb95eca0f69(&x);
  v3 = core::fmt::ArgumentV1::new_display::hea555bb95eca0f69(&index.end);
  args = v6;
  v16 = v3;
  core::fmt::Arguments::new_v1::h9b4392912f88443b(&p_args, __PAIR128__(2LL, &stru_9ACE8), __PAIR128__(2LL, &args));
  alloc::fmt::format::h3d488478954809ff(&v13, &p_args);
  result = retstr;
  *retstr = v13;
  return result;
```

Solver
--------
So out task to decode the provided file is to find the random key and unshuffle the provided file.
Since the encrypted file is an elf and we know magic numbers we can easily recover the key and write a method to unshuffle the bytes 

solver.py
```py
def findKey(encBytes):
    return encBytes[-1]

def decStage2(encbytes):
    result = []

    for i in range(0, len(encbytes), 3):
        result.append(encbytes[i + 1])
        result.append(encbytes[i + 2])
        result.append(encbytes[i])

    return result

def decStage1(encBytes, xorKey):
    for i in range(len(encBytes)):
        encBytes[i] ^= xorKey

    return encBytes

with open('flagGen.elf.enc', 'rb') as f:
    with open('flagGen.elf', 'wb') as g:
        encBytes = f.read()
        xorKey = findKey(encBytes)
        encBytes = decStage2(encBytes)
        encBytes = decStage1(encBytes, xorKey)
        g.write(bytes(encBytes))
```
Finally, we get our flag printer elf and executing it we get the flag

![image](https://github.com/YoungFlexerGR/challDev/assets/82509480/5c2a7f7a-edc2-4b3d-9343-2bec6d595d3e)













