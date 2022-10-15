const express = require("express")
const mongoose = require("mongoose")
const app = express()
var port = process.env.PORT || 8088
app.use(express.json())
app.use(express.urlencoded({ extended: true }))


//Set up default mongoose connection
const uri = "mongodb+srv://m001-student:admin@sandbox.bo4da.mongodb.net/mytranslationdb?retryWrites=true&w=majority";
mongoose.connect(uri, { useNewUrlParser: true, useUnifiedTopology: true },
    (err) => { if (err) { console.log(err) } else { console.log("connected to db") } })
// async function main() {
//     await mongoose.connect(uri,{ useNewUrlParser: true, useUnifiedTopology: true });
//   }

//creating schema model

const tSchema = new mongoose.Schema({
    region: { type: String },
    english: { type: String, required: true },
    saraiki: { type: String, required: true },
}, { timestamps: true });
const translate = mongoose.model('Translations', tSchema);
// const fluffy = new translate({_id:1,region:'Multan',english: 'fluffy','saraiki':'sss' });
// console.log(fluffy.region,fluffy.english,fluffy.saraiki)

app.get("/translations", (req, res) => {
    translate.find({}, (err, result) => {
        if (err) {
            console.log(err)
        } else {
            res.json(result)
        }
    })
})

// app.get("/getmultan",(req,res)=>{
//     translate.find({region:"Multan"},(err,result)=>{
//         if(err){
//             console.log(err)
//         }else{
//             res.json(result)
//             // console.log(result.map(v =>v.english))
//             // console.log(result.map(v =>v.saraiki))

//         }
//     })
// })
app.post("/newtranslation", (req, res) => {
    if (req.body.english === "" || req.body.english === undefined || req.body.english === null) {
        return res.json({ message: "English is required" })
    }
    else if (req.body.saraiki === "" || req.body.saraiki === undefined || req.body.saraiki === null) {
        return res.json({ message: "Saraiki is required" })
    }
    else {
        const newts = new translate(req.body)
        newts.save((err, result) => {
            if (err) {
                console.log(err)
            } else {
                res.json(result)
            }
        })
    }
})

app.put("/updatetranslation",async (req,res)=>{
if (req.body.english === "" || req.body.english === undefined || req.body.english === null) {
    return res.json({ message: "English is required" })
}
else if (req.body.saraiki === "" || req.body.saraiki === undefined || req.body.saraiki === null) {
    return res.json({ message: "Saraiki is required" })
}
else{
    const doc = await translate.findByIdAndUpdate(
            {_id:req.body._id},
            {
                ...req.body,
            }
    ,{new:true})
    return res.json(doc)
        }
})
//update by id again
app.put("/updatetranslation/:_id",async (req,res)=>{
    if (req.body.english === "" || req.body.english === undefined || req.body.english === null) {
        return res.json({ message: "English is required" })
    }
    else if (req.body.saraiki === "" || req.body.saraiki === undefined || req.body.saraiki === null) {
        return res.json({ message: "Saraiki is required" })
    }
    else{
        const doc = await translate.findByIdAndUpdate(
                {_id:req.body._id},
                {
                    ...req.body,
                }
        ,{new:true})
        return res.json(doc)
            }
    })
app.delete("/deletetranslation/:_id", (req,res)=>{
    translate.findByIdAndDelete(req.params._id,(err,success)=>{
        if(err) return res.json(err)
        return res.json(success)
    })

})
app.listen(port, () => {
    console.log("Server start at port:", port)
})