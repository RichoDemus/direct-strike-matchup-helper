const imp = import("../pkg/index.js");

export const from_wasm = async function load() {
    const {greet} = await imp
        // .then(obj => {
        //     const value = obj.greet();
        //     console.log("from js:",value);
        //     return obj;
        // })
        .catch(console.error);

    console.log("also js:", greet());
    return greet();
};

export const objFromWasm = async () => {
    const {get_object} = await imp;
    return get_object();
};

export const throwException = async () => {
  const {throw_exception} = await imp;
  return throw_exception;
};

export const get_matchups = async () => {
    const {get_matchups} = await imp;
    return get_matchups();
}

// load();
