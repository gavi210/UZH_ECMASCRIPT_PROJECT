const double_module_file = 'file:///Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/deno-module-test/side_modules/double.mjs';
const double_module = await import(double_module_file);

function run(n) {
    for (let i = 0; i < 10; i++) {
        double_module.double(n);
    }
}

for (let i = 0; i < 1000000; i++) {
    run(i);
}

Deno.core.print(String(import.meta.url))



