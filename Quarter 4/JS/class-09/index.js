//import express from "express";
const express = require("express")
const app = express()
//URI /App or /PIAC
app.get("/", (request, response) => {
    response.send("Hello World this express homepage")
})

app.get("/Class8", (request, response) => {
    response.send("First Page of class 8")
})
// Lets build a student array and show in get req
var employee = [
    { id: 1, name: "Farjad" },
    { id: 2, name: "Raheel" },
    { id: 3, name: "Adeel Cheema" }

]

app.get("/employee", (request, response) => {
    //response.send(JSON.stringify(employee))
    var result = `<table border="1">
    <tr><th>ID</th><th>Name</th></tr>`
    employee.map((value) => {
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
    app.use(express.json())
    app.post("/employee", (request, response) => {
        var student = {
            id: employee.length + 1,
            name: request.body.name
        }
        employee.push(student)
        response.send("New record pushed in employee")
    })
    app.get("/employee/:id", (request, response) => {
        // var student = {
        //     id: request.params.id,
        // }
        // response.send(student)
        var student = employee.find(i => i.id === parseInt(request.params.id))

        console.log(student)
        response.send(student)
    })
    app.put("/employee/:id", (request, response) => {
        var student = employee.find(i => i.id === parseInt(request.params.id))
        student.name = request.body.name
        response.send("Record is updated")
    })

    response.send(result)
})

app.delete("/employee/:id", (request, response) => {
    var student = employee.find(i => i.id === parseInt(request.params.id))
    var index = employee.indexOf(student)
    employee.splice(index, 1)
    response.send("Record is Deleted")
})


const port = 3000
app.listen(port, () => {
    console.log("The server is up and running at http://localhost:" + port)
})