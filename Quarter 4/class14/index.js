
const { WebhookClient } = require("dialogflow-fulfillment")
const express = require("express")
const myrequest = require('request')
const app = express()
const url = "http://api.weatherstack.com/current?access_key=d11feb71c27fc6c63749c6a42d5d7198&query="
app.get("/", (request, response) => {
    response.send("Hello World")
})

app.use(express.json()) //middleware to interact

app.post("/webhook", (request, response) => {
    const agent = new WebhookClient({ request: request, response: response })
    function welcome(agent) {
        console.log("incoming request fetched from local server")
        console.log(agent.parameters)
        var params = agent.parameters
        var num1 = params.num01
        var num2 = params.num02
        var result = num1 + num2
        agent.add(`Welcome to My First Agent with Express`);
        agent.add(`Addition Result of Numbers is: ${result}`);
    }

    async function temp(agent) {
        var params = agent.parameters
        var city = params.city
        let promise = new Promise((resolve, reject) => {
            myrequest.get(url + city, function (myerr, myres, mybody) {
                if (myerr) {
                    return 'Error, please try again'
                }

                else {
                    let weather = JSON.parse(mybody)
                    resolve(weather['current']['temperature'])
                }
            })
        }).catch(() => {
            console.log('Error')
        })
        try {
            let temp = await promise.then((res) => {
                const data = res
                return data
            })
            console.log(temp)
            
            agent.add(`Welcome to Temprature Intent`);
            agent.add(`City is: ${city}`);
            agent.add(
                `It is ${temp} degrees Celcius in ${city}`
            )

        } catch (err) {
            console.error("Something went wrong")
            console.error(err)
        }

    }
    let intentMap = new Map()
    intentMap.set("Default Welcome Intent", welcome) //Name of your intent in dialogflow
    intentMap.set("Temp-City", temp) //Name of your intent in dialogflow

    agent.handleRequest(intentMap)
})


const port = process.env.PORT || 3000
app.listen(port, () => {
    console.log("server is up and listening at http://localhost:" + port)
})