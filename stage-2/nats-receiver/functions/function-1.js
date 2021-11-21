const module_name = '/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/double.mjs';
const module = await import(module_name);

async function run() {
    let m = module.double(1000000000) ;

    for (let i = 0; i < m; i++) {
        //Deno.core.print("Iterating in loop...\n")
        ;
    }
}

//let obj = JSON.parse(Deno.args[1]);
run();