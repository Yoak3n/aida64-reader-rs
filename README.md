## Example

```toml
aida64-reader-rs = "*"
```

```rs
use std::collections::HashMap;
use aida64_reader_rs::shm;

fn main() {
    loop {
        let mut dictionary:HashMap<&str,&str> = HashMap::new();
        match shm::read_from_shared_memory()  {
            Ok(datas) => {
                datas.iter().for_each(|d| {
                    dictionary.insert(d.id.as_str(),d.value.as_str());  
                });
                println!("{:?}",dictionary);
            },
            Err(e) => panic!("{}",e),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
```