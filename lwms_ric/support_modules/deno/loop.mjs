// loop module for deno_core
function double(num) {
  return num * 2;
}

function loop(iterations) {
  for(let i = 0; i < iterations; i++) {
    double(i);
  }
  return 1;
}

export { loop };