use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
    .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "
    Block Mined
    Block Mined
    Block Mined
    true
    Blockchain {
        blocks: [
            Block {
                timestamp: 0,
                transaction: [
                    Transaction {
                        sender: Some(
                            PublicKey(CompressedEdwardsY: [33, 222, 128, 214, 78, 199, 230, 159, 3, 118, 163, 7, 116, 131, 28, 166, 170, 88, 45, 4, 39, 136, 156, 87, 223, 213, 57, 97, 151, 250, 198, 35]), EdwardsPoint{
                                    X: FieldElement51([1249628666188193, 1951369309648690, 261688874330438, 201325000849265, 920523141057359]),
                                    Y: FieldElement51([1515355365589136, 1726741697100698, 841715092371567, 10794825979216, 1325282611198755]),
                                    Z: FieldElement51([1138859552194776, 1427204751956123, 1123758225984774, 328442094993671, 945556191056491]),
                                    T: FieldElement51([666935372195589, 235002044581461, 121636660035621, 1923759395406747, 687869415487837])
                            }),
                        ),
                        receiver: Some(
                            PublicKey(CompressedEdwardsY: [163, 182, 218, 133, 95, 54, 46, 94, 172, 192, 148, 226, 2, 61, 100, 8, 232, 133, 8, 114, 6, 100, 63, 181, 64, 97, 195, 92, 41, 192, 7, 110]), EdwardsPoint{
                                    X: FieldElement51([1611947875166233, 2226428294774774, 2172969421366996, 113216869063618, 1093569990213709]),
                                    Y: FieldElement51([837327635295472, 24284499667040, 345325430001312, 1065682412929688, 1019219710987435]),
                                    Z: FieldElement51([1301398358200999, 1800731407702477, 14483234674832, 1649615957406835, 1948581111470397]),
                                    T: FieldElement51([1513480919132594, 1041123078379394, 1059663681851732, 1411025103622590, 1500598713317410])
                            }),
                        ),
                        amount: 2000.0,
                        signature: Some(
                            ed25519::Signature(4439B0A7690A9A2B6A459060729AF0B72DA6EB1426A90858FA08D95C1521C184869A0EC4B3AA83813DBFF0A9EA5993FD452248E012F7F6EFA148F69D62CE3F0D),
                        ),
                    },
                    Transaction {
                        sender: None,
                        receiver: Some(
                            PublicKey(CompressedEdwardsY: [29, 228, 10, 163, 84, 40, 189, 150, 12, 111, 240, 219, 150, 145, 186, 237, 97, 135, 99, 7, 222, 76, 120, 65, 179, 14, 51, 111, 252, 65, 51, 73]), EdwardsPoint{
                                    X: FieldElement51([732925358274514, 1633974664226504, 268901694247487, 275496735858855, 909903483415779]),
                                    Y: FieldElement51([1913227301909583, 799261522253668, 1722992237042639, 2149411747973202, 651471957271646]),
                                    Z: FieldElement51([1856585111006737, 233982145254692, 873430566546387, 1119407757884884, 1341383224391599]),
                                    T: FieldElement51([1858766701809436, 2066495796372735, 2107478425253924, 1524397281908932, 2022647575387244])
                            }),
                        ),
                        amount: 100.0,
                        signature: None,
                    },
                ],
                nonce: 7,
            },
            Block {
                timestamp: 0,
                transaction: [
                    Transaction {
                        sender: Some(
                            PublicKey(CompressedEdwardsY: [126, 4, 253, 231, 9, 99, 181, 101, 225, 115, 134, 213, 149, 103, 71, 54, 169, 210, 205, 82, 30, 216, 61, 31, 41, 176, 32, 228, 91, 23, 167, 129]), EdwardsPoint{
                                    X: FieldElement51([357857192651013, 1331826589692560, 149097046556787, 37348905137880, 1528860354008508]),
                                    Y: FieldElement51([602635256936925, 386001006290012, 2228937940025379, 869115669462764, 1950695096320407]),
                                    Z: FieldElement51([2133196178251063, 622167199279266, 1050805487476053, 175715677968864, 1225322382214022]),
                                    T: FieldElement51([398862071001776, 1822227836133800, 1705580007937751, 810307398211974, 1426533324381566])
                            }),
                        ),
                        receiver: Some(
                            PublicKey(CompressedEdwardsY: [217, 191, 100, 35, 189, 70, 250, 155, 131, 75, 37, 91, 124, 62, 33, 49, 79, 207, 225, 52, 20, 77, 120, 65, 101, 36, 113, 204, 185, 25, 120, 57]), EdwardsPoint{
                                    X: FieldElement51([414504336603271, 949825942262386, 1194759300296357, 1280450527135071, 1410417149037716]),
                                    Y: FieldElement51([491662180802232, 813884314654902, 1821081659098351, 845341206158130, 1861981046811979]),
                                    Z: FieldElement51([1546095259449979, 553179471796043, 227218635073627, 769923663636615, 1783855917076467]),
                                    T: FieldElement51([674942418033028, 631891861470978, 309193413739986, 556790451422656, 152857935024322])
                            }),
                        ),
                        amount: 2500.0,
                        signature: Some(
                            ed25519::Signature(63CBACA41F76D11A17482E02EF2C8F35C598BB92F1A093A06F44162FE4F0EEE924D6B4DF6DC040B2518ECFC3CE45D6F5C468154E84D18E9A1C47170F71636C07),
                        ),
                    },
                    Transaction {
                        sender: None,
                        receiver: Some(
                            PublicKey(CompressedEdwardsY: [29, 228, 10, 163, 84, 40, 189, 150, 12, 111, 240, 219, 150, 145, 186, 237, 97, 135, 99, 7, 222, 76, 120, 65, 179, 14, 51, 111, 252, 65, 51, 73]), EdwardsPoint{
                                    X: FieldElement51([732925358274514, 1633974664226504, 268901694247487, 275496735858855, 909903483415779]),
                                    Y: FieldElement51([1913227301909583, 799261522253668, 1722992237042639, 2149411747973202, 651471957271646]),
                                    Z: FieldElement51([1856585111006737, 233982145254692, 873430566546387, 1119407757884884, 1341383224391599]),
                                    T: FieldElement51([1858766701809436, 2066495796372735, 2107478425253924, 1524397281908932, 2022647575387244])
                            }),
                        ),
                        amount: 100.0,
                        signature: None,
                    },
                ],
                nonce: 62,
            },
            Block {
                timestamp: 0,
                transaction: [
                    Transaction {
                        sender: Some(
                            PublicKey(CompressedEdwardsY: [217, 191, 100, 35, 189, 70, 250, 155, 131, 75, 37, 91, 124, 62, 33, 49, 79, 207, 225, 52, 20, 77, 120, 65, 101, 36, 113, 204, 185, 25, 120, 57]), EdwardsPoint{
                                    X: FieldElement51([414504336603271, 949825942262386, 1194759300296357, 1280450527135071, 1410417149037716]),
                                    Y: FieldElement51([491662180802232, 813884314654902, 1821081659098351, 845341206158130, 1861981046811979]),
                                    Z: FieldElement51([1546095259449979, 553179471796043, 227218635073627, 769923663636615, 1783855917076467]),
                                    T: FieldElement51([674942418033028, 631891861470978, 309193413739986, 556790451422656, 152857935024322])
                            }),
                        ),
                        receiver: Some(
                            PublicKey(CompressedEdwardsY: [48, 253, 179, 23, 21, 125, 159, 54, 46, 188, 80, 192, 241, 95, 115, 201, 8, 103, 76, 28, 27, 102, 78, 23, 102, 211, 70, 45, 169, 156, 83, 114]), EdwardsPoint{
                                    X: FieldElement51([1336806015277703, 2130240682427499, 1642443544398942, 1005599009360199, 1803011975884920]),
                                    Y: FieldElement51([1157476925551127, 893630273838285, 524704587243927, 332375216513963, 1053661050062196]),
                                    Z: FieldElement51([1127534770584352, 1374037626581358, 1056683814188453, 1685132437875426, 2006867369460174]),
                                    T: FieldElement51([1405459591389766, 1939294030804769, 428294204352891, 492042567005481, 628263524545351])
                            }),
                        ),
                        amount: 1000.0,
                        signature: Some(
                            ed25519::Signature(DD925BE893444C0756518423406AD99B156DFF35138ED3633C6B7B11517FD2DF23DB86C1EC8C1C2FA3EB2602D00F8B8519CBF9D44E9E34E82443E6CE05551702),
                        ),
                    },
                    Transaction {
                        sender: None,
                        receiver: Some(
                            PublicKey(CompressedEdwardsY: [29, 228, 10, 163, 84, 40, 189, 150, 12, 111, 240, 219, 150, 145, 186, 237, 97, 135, 99, 7, 222, 76, 120, 65, 179, 14, 51, 111, 252, 65, 51, 73]), EdwardsPoint{
                                    X: FieldElement51([732925358274514, 1633974664226504, 268901694247487, 275496735858855, 909903483415779]),
                                    Y: FieldElement51([1913227301909583, 799261522253668, 1722992237042639, 2149411747973202, 651471957271646]),
                                    Z: FieldElement51([1856585111006737, 233982145254692, 873430566546387, 1119407757884884, 1341383224391599]),
                                    T: FieldElement51([1858766701809436, 2066495796372735, 2107478425253924, 1524397281908932, 2022647575387244])
                            }),
                        ),
                        amount: 100.0,
                        signature: None,
                    },
                ],
                nonce: 308,
            },
        ],
        unmined_transactions: [],
        mining_reward: 100.0,
    }";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}