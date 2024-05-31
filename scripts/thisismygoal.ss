// This Is My Goal
// (to be able to execute an overly complicated fizzbuzz program)
// (using my own interpreter)

const fizz = "fizz";
const buzz = "buzz";

function fizzBuzz(num: number) {
    let mod3 = num % 3;
    let mod5 = num % 5;

    if (mod3 == 0 && mod5 == 0) {
        return [fizz, buzz].join("");
    }
    if (mod3 == 0) {
        return fizz;
    }
    if (!mod5) {
        return buzz;
    }
    return "";
}

for(let i = 1; i <= 50; i++) {
    print(fizzBuzz(i))
}
