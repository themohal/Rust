const { WebhookClient } = require("dialogflow-fulfillment")
const express = require("express")
const app = express()


app.get("/", (request, response) => {
    response.send("Hello World")
})


// app.use(json()) //middleware to interact
app.post("/webhook", express.json(), (request, response) => {
    const agent = new WebhookClient({ request: request, response: response })
    function welcome(agent) {
        agent.add(`Welcome to My First Agent with Express`);
    }
    let intentMap = new Map()
    intentMap.set("Default Welcome Intent", welcome) //Name of your intent in dialogflow
    agent.handleRequest(intentMap)
})


const port = 3000
app.listen(port, () => {
    console.log("server is up and listening at http://localhost:" + port)
})