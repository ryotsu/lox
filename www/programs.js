export let programs = {
  "Hello World": `print "Hello World!";`,
  "Loops": `fun triangle(num) {
  for(var i = 0; i < num; i = i + 1) {
    var stars = "";
    var spaces = "";
    for(var j = i; j < num; j = j + 1) {
      spaces = spaces + " ";
    }
    for(var j = 0; j <= i; j = j + 1) {
      stars = stars + "* ";
    }
    print spaces + stars;
  }
}

triangle(20);
`,

  "Fibonacci": `fun fib(num) {
  if (num < 2) {
    return num;
  }

  return fib(num - 1) + fib(num - 2);
}

print fib(20);
`,

  "Factors": `fun remainder(num, divisor) {
  while(num - divisor >= 0) {
    num = num - divisor;
  }
  return num;
}

fun prime_factors(num) {
  var i = 2;
  while (num > i * i) {
    while (remainder(num, i) == 0) {
      num = num / i;
      print i;
    }
    i = i + 1;
  }
  if (num > 1) {
    print num;
  }
}

prime_factors(6300);`,

  "Closures": `fun fib_gen() {
  var a = 0;
  var b = 1;
  fun fib() {
    return a = b = a + b;
  }
  return fib;
}

var fib = fib_gen();

for (var i = 0; i <= 20; i = i + 1) {
 print fib();
}`
}
