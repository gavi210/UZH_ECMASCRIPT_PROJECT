const module_name = './double.mjs'
const module = await import(module_name);

async function run() {
    let m = module.double(1000000000) ;

    for (let i = 0; i < m; i++) {
        ;
    }
}

run();