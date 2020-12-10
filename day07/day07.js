const readline = require('readline');

const fs = require('fs');
const file = fs.readFileSync('input', { encoding: 'utf8' });
const lines = file.split("\n");

const re = /(\d+) (.+?) bags?/g;

const bagsByColour = {};

function getOrCreateBag(colour) {
    if (!bagsByColour[colour]) {
        bagsByColour[colour] = {
            colour: colour,
            parents: [],
            children: [],
        };
    }
    return bagsByColour[colour];
}

lines.forEach(function(line) {
    const parentAndChildren = line.split(" bags contain ");
    const parentColour = parentAndChildren[0];
    const childrenStr = parentAndChildren[1];
    const children = [];
    var matches = re.exec(childrenStr);
    while (matches) {
        children.push({ number: parseInt(matches[1], 10), colour: matches[2] });
        matches = re.exec(childrenStr);
    }
    
    const parentBag = getOrCreateBag(parentColour);
    parentBag.children = children;

    children.forEach(function(child) {
        const childBag = getOrCreateBag(child.colour);
        if (childBag.parents.indexOf(parentColour) === -1) {
            childBag.parents.push(parentColour);
        }
    })
});

const ancestors = ['shiny gold']; // Add shiny gold as an ancestor in case it's a parent somewhere
const queue = ['shiny gold'];
while (queue.length > 0) {
    var colour = queue.shift();
    var bag = bagsByColour[colour];
    bag.parents.forEach(function(parent) {
        if (ancestors.indexOf(parent) === -1) {
            ancestors.push(parent);
            queue.push(parent);
        }
    });
}

console.log('Part 1', ancestors.length - 1); // Part 1 - don't count shiny gold

// Assume no cycles in children...
function countDescendents(bag) {
    if (!bag.children || bag.children.length === 0) {
        return 0;
    }
    var count = 0;
    bag.children.forEach(function(child) {
        count += child.number;
        count += child.number * countDescendents(bagsByColour[child.colour]);
    });
    return count;
}
const part2 = countDescendents(bagsByColour['shiny gold']);
console.log('Part 2', part2);