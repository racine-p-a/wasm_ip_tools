import init, {
    convert_dotted_decimals_to_binary,
    convert_hexadecimals_to_binary,
    convert_decimals_to_binary,
    convert_octals_to_binary,
    convert_ip_binary_to_hexadecimals,
    convert_ip_binary_to_decimals,
    convert_ip_binary_to_octals,
    convert_ip_binary_to_dotted_decimals
} from "./pkg/wasm_ip_tools.js";

const results = {
    'dottedDecimals':'',
    'binary':'',
    'dottedHexadecimals':'',
};

init().then(() => {
    document.getElementById('convertFromDottedDecimals').addEventListener('click', function () {
        const ipDottedDecimals = document.getElementById(('dottedDecimals')).value;
        results.dottedDecimals = ipDottedDecimals;

        const ipBinary = convert_dotted_decimals_to_binary(ipDottedDecimals);
        console.log(ipBinary);
        document.getElementById('binary').value = ipBinary;

        const ipHexadecimals = convert_ip_binary_to_hexadecimals(ipBinary);
        console.log("ipHexadecimals : " + ipHexadecimals);
        document.getElementById('dottedHexadecimals').value = ipHexadecimals;

        const ipDecimals = convert_ip_binary_to_decimals(ipBinary);
        console.log(ipDecimals);
        document.getElementById('decimals').value = ipDecimals;

        const ipOctals = convert_ip_binary_to_octals(ipBinary);
        console.log(ipOctals);
        document.getElementById('octals').value = ipOctals;
    });

    document.getElementById('convertFromBinary').addEventListener('click', function () {
        const ipBinary = document.getElementById(('binary')).value;

        const ipDottedDecimals = convert_ip_binary_to_dotted_decimals(ipBinary);
        console.log(ipDottedDecimals);
        document.getElementById('dottedDecimals').value = ipDottedDecimals;

        const ipHexadecimals = convert_ip_binary_to_hexadecimals(ipBinary);
        console.log(ipHexadecimals);
        document.getElementById('dottedHexadecimals').value = ipHexadecimals;

        const ipDecimals = convert_ip_binary_to_decimals(ipBinary);
        console.log(ipDecimals);
        document.getElementById('decimals').value = ipDecimals;

        const ipOctals = convert_ip_binary_to_octals(ipBinary);
        console.log(ipOctals);
        document.getElementById('octals').value = ipOctals;
    });

    document.getElementById('convertFromDottedHexadecimals').addEventListener('click', function () {
        const ipDottedHexaDecimals = document.getElementById(('dottedHexadecimals')).value;

        const ipBinary = convert_hexadecimals_to_binary(ipDottedHexaDecimals);
        console.log(ipBinary);
        document.getElementById('binary').value = ipBinary;

        const ipDottedDecimals = convert_ip_binary_to_dotted_decimals(ipBinary);
        console.log(ipDottedDecimals);
        document.getElementById('dottedDecimals').value = ipDottedDecimals;

        const ipDecimals = convert_ip_binary_to_decimals(ipBinary);
        console.log(ipDecimals);
        document.getElementById('decimals').value = ipDecimals;

        const ipOctals = convert_ip_binary_to_octals(ipBinary);
        console.log(ipOctals);
        document.getElementById('octals').value = ipOctals;
    });

    document.getElementById('convertFromDecimals').addEventListener('click', function () {
        const ipDecimals = document.getElementById(('decimals')).value;

        const ipBinary = convert_decimals_to_binary(ipDecimals);
        console.log(ipBinary);
        document.getElementById('binary').value = ipBinary;

        const ipDottedDecimals = convert_ip_binary_to_dotted_decimals(ipBinary);
        console.log(ipDottedDecimals);
        document.getElementById('dottedDecimals').value = ipDottedDecimals;

        const ipHexadecimals = convert_ip_binary_to_hexadecimals(ipBinary);
        console.log(ipHexadecimals);
        document.getElementById('dottedHexadecimals').value = ipHexadecimals;

        const ipOctals = convert_ip_binary_to_octals(ipBinary);
        console.log(ipOctals);
        document.getElementById('octals').value = ipOctals;
    });

    document.getElementById('convertFromoctals').addEventListener('click', function () {
        const ipOctals = document.getElementById(('octals')).value;

        const ipBinary = convert_octals_to_binary(ipOctals);
        console.log(ipBinary);
        document.getElementById('binary').value = ipBinary;

        const ipDottedDecimals = convert_ip_binary_to_dotted_decimals(ipBinary);
        console.log(ipDottedDecimals);
        document.getElementById('dottedDecimals').value = ipDottedDecimals;

        const ipHexadecimals = convert_ip_binary_to_hexadecimals(ipBinary);
        console.log(ipHexadecimals);
        document.getElementById('dottedHexadecimals').value = ipHexadecimals;

        const ipDecimals = convert_ip_binary_to_decimals(ipBinary);
        console.log(ipDecimals);
        document.getElementById('decimals').value = ipDecimals;

    });

});

