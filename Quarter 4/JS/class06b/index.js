var a = 10
var b = 20

if (a < b) {
    console.log("A less than B")
} else {
    console.log("B is less than A")
}

op = "+"
switch (op) {
    case '+':
        console.log(a + b)
    case '+':
        console.log(a + b)
    case '+':
        console.log(a + b)
    default: console.log("Invalid Operator")
}
result = "*"

for (let index = 0; index < 6; index++) {
    j = 0
    while (j < index) {
        console.log(result)
        result = result+"*"
        j++
    }
}

myArray = ['Farjad','Rameez','Shahzaib'];
myArray.forEach(element => {
    console.log(element)
    
});
for (let index = 0; index < myArray.length; index++) {
    const element = myArray[index];
    console.log("From For :"+ element)
    console.log(`From Second: ${element}`)
    
}