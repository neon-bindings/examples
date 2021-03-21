var addon = require('../index.node');

function getCern() {
    addon.getHtmlAsync("http://info.cern.ch/hypertext/WWW/TheProject.html", (err, result) => {
        console.log(err, result);
    });
}

getCern();
