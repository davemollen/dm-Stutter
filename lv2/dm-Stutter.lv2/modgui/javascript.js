function(event) {
  function handle_event(symbol, value) {
    switch (symbol) {
      case "auto":
        const chance = event.icon.find("[mod-port-symbol=chance]");
        const duration = event.icon.find("[mod-port-symbol=duration]");

        if(value == 1) {
          chance.removeClass("disabled");
          duration.removeClass("disabled");
        } else {
          chance.addClass("disabled");
          duration.addClass("disabled");
        }
        break;
      case "sync":
        const pulse = event.icon.find("[mod-port-symbol=pulse]");
        const tempo_factor = event.icon.find("[mod-port-symbol=tempo_factor]");

        if(value == 1) {
          pulse.addClass("hide");
          tempo_factor.removeClass("hide");
        } else {
          pulse.removeClass("hide");
          tempo_factor.addClass("hide");
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