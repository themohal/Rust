// console.log("Task 1")
// console.log("Task 2")
// console.log("Task 3")

// console.log("Task 1")
// setTimeout(() => {
//     console.log("Task 2")
// }, 3000);
// console.log("Task 3")

var condition = true
// callbacks
var prom = new Promise((resolve, reject) => {
    if (condition) {
        resolve(["Task Completed"])
    } else {
        reject(new Error("Task not Completed"))
    }
})

// prom.then(value => { return value }).then(value => { console.log(value) }).catch(err => { console.log(err) })
// Async Await
async function asyncfunc() {
    try {
        var result = await prom;
        console.log(result)
    }
    catch (error) {
        console.log(error)
    }
}
// asyncfunc()

//asyn call
(async () => {
    await asyncfunc()
})();