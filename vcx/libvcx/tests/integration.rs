extern crate vcx;
extern crate serde;
extern crate rand;

#[macro_use]
extern crate serde_json;


use rand::Rng;
use vcx::utils::cstring::CStringUtils;
use vcx::utils::libindy::return_types_u32;
use std::fs;
use std::io::Write;
use std::time::Duration;
use std::ffi::CString;
use vcx::settings;
use vcx::utils::constants::GENESIS_PATH;
use vcx::api::utils::vcx_agent_provision_async;
use vcx::api::vcx::{ vcx_init_with_config, vcx_shutdown };

pub fn get_sandbox() -> serde_json::Value {
    json!({
        "url": "https://agency-ea-sandbox.evernym.com",
        "did": "HB7qFQyFxx4ptjKqioEtd8",
        "verkey": "9pJkfHyfJMZjUjS7EZ2q2HX55CbFQPKpQ9eTjSAUMLU8",
        "seed": "000000000000000000000000Trustee1"
    })
}

pub fn get_dev() -> serde_json::Value {
    json!({
        "url": "https://enym-eagency.pdev.evernym.com",
        "did": "dTLdJqRZLwMuWSogcKfBT",
        "verkey": "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH",
        "seed": "000000000000000000000000Trustee1"
    })
}

pub fn create_genesis_txn_file() {
    let test_pool_ip = "127.0.0.1".to_string();

    let node_txns = vec![
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)];

    let txn_file_data = node_txns[0..4].join("\n");

    let mut f = fs::File::create(GENESIS_PATH).unwrap();
    f.write_all(txn_file_data.as_bytes()).unwrap();
    f.flush().unwrap();
    f.sync_all().unwrap();
}

fn provision_agent() -> Result<String, u32> {
    let mut rng = rand::thread_rng();
    let settings = get_sandbox();
    let config = json!({
            "wallet_name": "LIBVCX_SDK_WALLET",
            "agent_seed": format!("HANKHILL{}00000000001DIRECTION", rng.gen_range(1000,9999)),
            "enterprise_seed": settings["seed"],
            "wallet_key": "1234",
            "agency_url": settings["url"],
            "agency_did": settings["did"],
            "agency_verkey": settings["verkey"],
        });
    let config = CStringUtils::string_to_cstring(config.to_string());
    let cb = return_types_u32::Return_U32_STR::new().unwrap();
    vcx_agent_provision_async(cb.command_handle, config.as_ptr(), Some(cb.get_callback()));
    let vcx_config = cb.receive(Some(Duration::from_secs(10)))?.unwrap();
    let mut vcx_config:serde_json::Value = serde_json::from_str(&vcx_config).unwrap();
    let vcx_config = vcx_config.as_object_mut().unwrap();
    vcx_config.insert( "institution_logo_url".to_string(), json!("https://robohash.org/hankhill"));
    vcx_config.insert( "institution_name".to_string(), json!("Harlan Gas"));
    vcx_config.insert( "genesis_path".to_string(), json!(GENESIS_PATH));
    match serde_json::to_string(&vcx_config) {
        Ok(s) => Ok(s),
        Err(e) => Err(1),
    }
}

fn delete_indy_client(){
    use std::fs::remove_dir_all;
    use std::env::home_dir;
    use std::path::PathBuf;
    let p = match home_dir() {
        Some(path) => path,
        None => panic!("Cannot find home directory"),
    };
    let mut path = PathBuf::new();
    path.push(p);
    path.push(".indy_client");
    path.push("wallet");
    remove_dir_all(path).unwrap_or(());
}

fn init_vcx(vcx_config: &str) -> Result<(), u32> {
    let cb = return_types_u32::Return_U32::new().unwrap();
    let err = vcx_init_with_config(cb.command_handle,
                                   CString::new(vcx_config).unwrap().as_ptr(),
                                   Some(cb.get_callback()));
    cb.receive(Some(Duration::from_secs(10)))
}


#[cfg(test)]
mod tests {
    use super::*;
    use vcx::utils::error;

    #[cfg(feature = "sovtoken")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_error_codes() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::TEST_WALLET_KEY);
        delete_indy_client();
        create_genesis_txn_file();
        let vcx_config = provision_agent().unwrap();
        init_vcx(&vcx_config).unwrap();
        vcx_shutdown(false);
        assert_eq!(provision_agent().err(), Some(error::DID_ALREADY_EXISTS_IN_WALLET.code_num));
    }
}