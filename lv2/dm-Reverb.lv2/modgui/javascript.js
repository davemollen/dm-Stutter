function(event) {
  function handle_event(symbol, value) {
    switch (symbol) {
        case "reverse":
            const reverse = event.icon.find("[mod-port-symbol=reverse]");
            if(value == 1) {
              reverse.addClass("on");
            } else {
              reverse.removeClass("on");
            }
            break;
        default:
            break;
    }
  }

  if (event.type == 'start') {
    const ports = event.ports;
    for (const p in ports) {
      handle_event(ports[p].symbol, ports[p].value);
    }
  }
  else if (event.type == 'change') {  
    handle_event(event.symbol, event.value);
  }
}