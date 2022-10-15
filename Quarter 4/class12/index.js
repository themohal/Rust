const express = require("express")
const path = require("path")
const app = express()

app.use(express.urlencoded({ extended: true }))
app.use(express.json())


const employees = [
    { id: 1, name: "Farjad" },
    { id: 2, name: "Raheel" },
    { id: 3, name: "Adeel Cheema" }

]
// app.get('/', (req, res) => {
//     res.sendFile(__dirname + '/static/index.html');
//   });
  
//   // Route to Login Page
//   app.get('/login', (req, res) => {
//     res.sendFile(__dirname + '/static/login.html');
//   });

// Route to Homepage
app.get('/', function (req, res) {
    res.sendFile(path.join(__dirname, 'views/index.html'));
});

app.get("/all", (request, response) => {
    //response.send(JSON.stringify(employee))
    var result = `<table border="1">
    <tr><th>ID</th><th>Name</th></tr>`
    employees.map((value) => {
        result += `<tr>
        <td>${value.id}</td>
        <td>${value.name}</td>
        </tr>`
    })
    result += "</table>"
    //lets add new record
    //we also need middleware for the post method to interact with outside world 
    //body request we need middleware app.use(express.json()) for request only not response
    //to connect with server
    response.send(result)

})
app.post("/employee", (request, response) => {
    var employee = {
        id: employees.length + 1,
        name: request.body.name
    }
    employees.push(employee)
    // response.send("New record pushed in employee")
    // response.sendFile(path.join(__dirname, '/index.html'));
    response.redirect("/");

})
app.get("/employee/:id", (request, response) => {
    // var student = {
    //     id: request.params.id,
    // }
    // response.send(student)
    var employee = employees.find(i => i.id === parseInt(request.params.id))

    console.log(employee)
    response.send(employee)
})
app.put("/employee/:id", (request, response) => {
    var employee = employees.find(i => i.id === parseInt(request.params.id))
    employee.name = request.body.name
    response.send("Record is updated")
})



app.delete("/employee/:id", (request, response) => {
    var employee = employees.find(i => i.id === parseInt(request.params.id))
    var index = employees.indexOf(employee)
    employees.splice(index, 1)
    response.send("Record is Deleted")
})
var port = process.env.PORT || 8088


app.listen(port, () => {
    console.log("The server is up and running at" + port)
})