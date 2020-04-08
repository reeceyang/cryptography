use rug::Integer;

/*// returns an array with binary digits
fn toBinaryHelper(n,b):
    if n==0:
        b.reverse()
        return b
    else:
        return toBinaryHelper((n-n%2)/2,b+[n%2])

// returns an array with binary digits, without the need for the second argument
def toBinary(n):
    return toBinaryHelper(n,[])
*/

// returns the binary digits of n as string
pub fn to_binary_string(n: &Integer) -> String {
    //n_array = toBinary(n)
    //s = ''.join(map(str, nArray))
    format!("{:b}", n)
}

//returns the decimal to binary string n
pub fn to_decimal(n: &String) -> Integer {
    /*let mut dec = Integer::from(0);
    for i in 0..n.len() {
        dec += int(n[n.len() - i - 1])*(2**i)
    }
    dec*/
    // this might cause bugs later on
    Integer::from(isize::from_str_radix(n, 2).unwrap())
}


// take an array of integers
// translate to binary, buffer with leading zeros to "blockSize1"
// concatenate
// chop concatenation into chunks of "blockSize2"
// return array of decimals of the chunks
// IDEALLY, blockConvert(blockConvernt(array, m, n),n,m) = array
// problems arise with the last block which can have unknown size
pub fn block_convert(decimal_array_in: &Vec<Integer>, block_size_1: u32, block_size_2: u32) -> Vec<Integer> {
    let mut binary_array_1: Vec<String> = decimal_array_in.iter().map(|x| to_binary_string(x)).collect();

    binary_array_1 = binary_array_1.iter().map(|x|
        std::iter::repeat("0").take(block_size_1 as usize - x.len()).collect::<String>() + x).collect();
    //[''.join(['0' for i in range(blockSize1-len(n))])+n for n in binaryArray1]

    println!("{:?}", binary_array_1);
    let binary_string: String = binary_array_1.into_iter().map(|i| i.to_string()).collect::<String>();

    let number_of_blocks: u32 = (binary_string.len() as u32) / block_size_2;

    let mut binary_array_2: Vec<String> = Vec::new();
    for i in 0..number_of_blocks {
        binary_array_2.push(binary_string.chars().skip((block_size_2 * i) as usize).take(block_size_2 as usize).collect());
        //binary_array_2.push(binary_string[(block_size_2 * i) as i32..(block_size_2 * (i + 1)) as i32]);
    }

    if block_size_2 * number_of_blocks < binary_string.len() as u32 {
        binary_array_2.push(binary_string.chars().skip((block_size_2 * number_of_blocks) as usize)
            .take(binary_string.len() - ((block_size_2 * number_of_blocks) as usize)).collect());
    }

    println!("{:?}", binary_array_2);
    let decimal_array_2 = binary_array_2.iter().map(|x| to_decimal(x)).collect();
    decimal_array_2
}

/*
# EDIT THIS TO CONTAIN AN ACTUAL TEXT FILE
file=open(/Users/username/yourfile.txt,'rb')

#Read it the content of the file, store the byte codes in array "bytes".
bytes = []
with file as f:
    while True:
        byte = f.read(1)
        if not byte:
            break
        bytes.append(ord(byte))
file.close()

# bytes are 8 bits, so one blockSize is 8. If the other blockSize is a multiple of 8,
# you just have to kill the redundant zeros that may occur. This is easy for a text,
# but iffy for a binary file which may have legitimate zeros.
# A solution to this would be to transmit metadata about the blocksize of each block,
# in order to fill in the appropriate number of leading zeros.
print bytes
new=blockConvert(bytes,8,32)
print new
old = blockConvert(new,32,8)
print old
*/
