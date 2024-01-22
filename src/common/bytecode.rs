use ethers::{prelude::Lazy, types::Bytes};

pub static REQUEST_BYTECODE: Lazy<Bytes> = Lazy::new(|| {
    "0x608060409080825260048036101561001657600080fd5b6000803560e01c631f69565f1461002c57600080fd5b3461014857602093846003193601126102035782356001600160a01b03811693908490036101ff576306fdde0360e01b855282858281875afa9485156101f35783956101d7575b5081516395d89b4160e01b81529083828281885afa9182156101ca5784926101a6575b50825163313ce56760e01b81529487868381845afa95861561019c57908891869761015d575b5084516318160ddd60e01b815292839182905afa938415610152578094610119575b505061010a6100fd9660ff92845198899860808a5260808a019061022a565b918883039089015261022a565b93169084015260608301520390f35b909193508682813d831161014b575b610132818361024f565b810103126101485750519161010a6100fd6100de565b80fd5b503d610128565b8351903d90823e3d90fd5b8281939298503d8311610195575b610175818361024f565b81010312610191575160ff8116810361019157879095386100bc565b8480fd5b503d61016b565b84513d87823e3d90fd5b6101c39192503d8086833e6101bb818361024f565b810190610287565b9038610096565b50505051903d90823e3d90fd5b6101ec9195503d8085833e6101bb818361024f565b9338610073565b505051903d90823e3d90fd5b8280fd5b5080fd5b60005b83811061021a5750506000910152565b818101518382015260200161020a565b9060209161024381518092818552858086019101610207565b601f01601f1916010190565b90601f8019910116810190811067ffffffffffffffff82111761027157604052565b634e487b7160e01b600052604160045260246000fd5b6020818303126102f357805167ffffffffffffffff918282116102f357019082601f830112156102f357815190811161027157604051926102d2601f8301601f19166020018561024f565b818452602082840101116102f3576102f09160208085019101610207565b90565b600080fdfea264697066735822122004fbd047c788ee9f88c1adbdb92195b7b34a4454850b26610c1bca2cfdee742264736f6c63430008140033".parse().unwrap()
});

pub static SANDOOO_BYTECODE: Lazy<Bytes> = Lazy::new(|| {
    "0x6080604052600436101561001e575b361561001c5761001c61011c565b005b6000803560e01c63b29a814014610035575061000e565b346100b25760403660031901126100b257806001600160a01b0360043581811681036100e05761008b602435927f00000000000000000000000000000000000000000000000000000000000000001633146100e4565b82811591826000146100b55750506001146100a35750f35b81808092335af1156100b25780f35b80fd5b60449250908093916040519263a9059cbb60e01b845233600485015260248401525af1156100b25780f35b5050fd5b156100eb57565b60405162461bcd60e51b81526020600482015260096024820152682727aa2fa7aba722a960b91b6044820152606490fd5b610150337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316146100e4565b60405160009043823560c01c03610217576008600482019160248101925b36831061017c575050505050565b823560f81c926060906001810135821c916015820135901c9487806044878260298701359a6069604989013598019b63a9059cbb60e01b8452898b528d525af1156102135784888094819460a49463022c0d9f60e01b855280600014610207576001146101fc575b50306044840152608060648401525af161016e578480fd5b8288528a52386101e4565b508752818a52386101e4565b8780fd5b5080fdfea26469706673582212205780a691eeddd4e610da8935f362884e1cf8e5b54b5aba51fbfafbb95427a2b864736f6c63430008140033".parse().unwrap()
});
