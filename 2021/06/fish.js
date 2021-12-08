const fs = require('fs')

const fish = fs.readFileSync('/aoc/in', 'utf8').split(',').map(x => Number(x))
const days = process.argv[2]

const fish_ages = fish.reduce((acc, x) => {
    acc[x] = acc[x] + 1
    return acc
}, new Array(9).fill(0))

for (var k = 0; k < days; k++) {
    const cycled_fish = fish_ages.shift()
    fish_ages[6] = fish_ages[6] + cycled_fish
    fish_ages[8] = cycled_fish
}
console.log(fish_ages.reduce((acc, x) => acc + x, 0))