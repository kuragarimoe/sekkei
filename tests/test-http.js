// MOCK HTTP SERVER THINGY
const express = require("express");
const app = express();

app.use(express.json())

app.all("/test", (req, res) => {
    res.json({
        message: "Hewwo"
    });
});

app.listen(9898);