const module_name = './double.mjs';
const module = await import(module_name);

async function run() {
    let m = module.double(1000000000) ;

    for (let i = 0; i < m; i++) {
        //Deno.core.print("Iterating in loop...\n")
        ;
    }
}

//let obj = JSON.parse(Deno.args[1]);
//console.log("About to run function-1 with input arguments: ", Deno.args);
run();