use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Name(usize);

pub enum Data {
    // A Name created by user (found in source code somewhere)
    User {
        name: String,
    },
    // A Name created by the compiler (for explicating and flattening)
    CompilerTmp,
}

pub struct Map {
    src_names: HashMap<String, Name>,
    names: HashMap<Name, Data>,
    // Next Name index
    next: usize,
}

impl Map {
    pub fn new() -> Map {
        Map {
            src_names: HashMap::new(),
            names: HashMap::new(),
            next: 0,
        }
    }

    fn gen_name(&mut self) -> Name {
        let name = Name(self.next);
        self.next += 1;
        name
    }

    pub fn insert_name(&mut self, name: String) -> Name {
        // if name already in map, just return Name
        if let Some(&name) = self.src_names.get(&name) {
            assert!(self.names.contains_key(&name));
            return name
        }
        // otherwise,
        // create a new Name
        let handle = self.gen_name();
        // insert src_name
        // note that we're inducing a copy here! blerg!
        self.src_names.insert(name.clone(), handle);
        // insert name data
        let data = Data::User { name };
        self.names.insert(handle, data);

        handle
    }

    pub fn create_tmp(&mut self) -> Name {
        // generate a new Name
        let handle = self.gen_name();
        // insert name data
        self.names.insert(handle, Data::CompilerTmp);

        handle
    }

    pub fn name_data(&self, handle: Name) -> &Data {
        self.names.get(&handle).expect("Map doesn't contain name")
    }
}
