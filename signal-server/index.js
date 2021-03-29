const express = require('express'); 
const { ExpressPeerServer } = require('peer'); 
const compression = require('compression'); 
const helmet = require('helmet');

const combined = function(){
    const arr1 = ["love", "affection", "appreciation", "devotion", "emotion", "fondness", "friendship", "infatuation", "lust", "passion", "respect", "taste", "tenderness", "yearning", "adulation", "allegiance", "amity", "crush", "delight", "worship", "amour", "soft-spot", "tender", "beloved", "cherished", "adored"]; 
    const arr2 = ["aries", "taurus", "gemini", "cancer", "leo", "virgo", "libra", "scorpio", "sagittarius", "capricorn", "aquarius", "pisces"];
    const arr3 = ["sachiel", "shamshel", "gaghiel", "israfel", "sanadalphon", "matarael", "sahaquiel", "ireul", "leliel", "bardiel", "zeruel", "Arael", "armisael", "tabris", "lilin"];
    const arr4 = ["earth", "mars", "saturn", "jupiter", "venus", "uranus", "mercury", "neptune", "pluto"];
    const combo = arr1.concat(arr2.concat(arr3.concat(arr4)));
    const signature = (Math.random().toString(36) + '0000000000000000000').substr(2, 16);
    return combo[Math.floor(Math.random()*combo.length)] + "-" + signature;
}; 


const app = express(); 

app.get('/', (req, res, next) => res.send('unused'));

const server = app.listen(9000); 

const peerConnections = ExpressPeerServer(server, {
    path: "/plane", 
    debug: true,
    generateClientId: combined,
}); 

app.use(compression());
app.use(helmet());
app.use('/astral', peerConnections);