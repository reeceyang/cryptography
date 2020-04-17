use rug::Integer;

// returns the binary digits of n as string
pub fn to_binary_string(n: &Integer) -> String {
    format!("{:b}", n)
}

//returns the decimal to binary string n
pub fn to_decimal(n: &String) -> Integer {
    // this might cause bugs later on
    Integer::from(isize::from_str_radix(n, 2).unwrap())
}


// take an array of integers
// return array of decimals of the chunks
// IDEALLY, blockConvert(blockConvernt(array, m, n),n,m) = array
// problems arise with the last block which can have unknown size
pub fn block_convert(decimal_array_in: &Vec<Integer>, block_size_1: u32, block_size_2: u32) -> Vec<Integer> {
    // translate to binary, buffer with leading zeros to "blockSize1"
    let mut binary_array_1: Vec<String> = decimal_array_in.iter().map(|x| to_binary_string(x)).collect();

    // pad with extra 0s
    binary_array_1 = binary_array_1.iter().map(|x|
        std::iter::repeat("0").take(block_size_1 as usize - x.len()).collect::<String>() + x).collect();
    //println!("{:?}", binary_array_1);

    // concatenate into a string
    let binary_string: String = binary_array_1.into_iter().map(|i| i.to_string()).collect::<String>();

    // chop concatenation into chunks of "blockSize2"
    let number_of_blocks: u32 = (binary_string.len() as u32) / block_size_2;

    let mut binary_array_2: Vec<String> = Vec::new();
    for i in 0..number_of_blocks {
        binary_array_2.push(binary_string.chars().skip((block_size_2 * i) as usize).take(block_size_2 as usize).collect());
    }

    if block_size_2 * number_of_blocks < binary_string.len() as u32 {
        binary_array_2.push(binary_string.chars().skip((block_size_2 * number_of_blocks) as usize)
            .take(binary_string.len() - ((block_size_2 * number_of_blocks) as usize)).collect());
    }
    //println!("{:?}", binary_array_2);

    // return array of decimals of the chunks
    let decimal_array_2 = binary_array_2.iter().map(|x| to_decimal(x)).collect();
    decimal_array_2
}
