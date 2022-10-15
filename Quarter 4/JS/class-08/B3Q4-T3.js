function add(a, b) {
    console.log("Addition is: " + (a + b))
}
function sub(a, b) {
    console.log("Addition is: " + (a - b))
}
function mul(a, b) {
    console.log("Addition is: " + (a * b))
}
function div(a, b) {
    if (b < 1) {
        console.log("Division By Zero Error")
    } else {
        console.log("Addition is: " + (a / b))
    }
}

function calculate(a, b, callback) {
    callback(a, b)
}
calculate(1, 0, div)