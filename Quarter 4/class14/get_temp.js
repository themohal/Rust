const request = require('request')
function temprature(city) {
    let promise = new Promise((resolve, reject) => {
        request(`http://api.weatherstack.com/current?access_key=d11feb71c27fc6c63749c6a42d5d7198&query=${city}`, (error, response, body) => {
            console.log(`Error ${error}`)
            console.log(`Response ${response}`)
            // console.log(`Body${body}`)
            resolve(body)
        })
        reject('Reject')
    })
    return promise
}

// city_name = 'Karachi'
// temprature(city_name).
// then(temp=>{
//     // console.log(temp)
//     var data_temp = JSON.parse(temp);
//     console.log(data_temp)
//     console.log(data_temp.current.temprature)
// })