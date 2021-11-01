const loop_module_file = 'file:///Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/lwms_ric/support_modules/side_modules/loop.mjs';
const loop_module = await import(loop_module_file);

Deno.core.print("Running test 01\n");

function run(n) {
    loop_module.loop(n);
}

for (let i = 0; i < 10000; i++) {
    run(i);
}



