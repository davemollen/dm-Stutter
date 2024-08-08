function(event) {
  function handle_event(symbol, value) {
    switch (symbol) {
      case "on":
        const on = event.icon.find("[mod-port-symbol=on]");
        if(value == 1) {
          on.addClass("on");
        } else {
          on.removeClass("on");
        }
        break;
      case "auto":
        const auto = event.icon.find("[mod-port-symbol=auto]");
        const chance = event.icon.find("[mod-port-symbol=chance]");
        const duration = event.icon.find("[mod-port-symbol=duration]");

        if(value == 1) {
          auto.addClass("on");
          chance.removeClass("disabled");
          duration.removeClass("disabled");
        } else {
          auto.removeClass("on");
          chance.addClass("disabled");
          duration.addClass("disabled");
        }
        break;
      case "trigger":
        const trigger = event.icon.find("[mod-port-symbol=trigger]");
        if(value == 1) {
          trigger.addClass("on");
        } else {
          trigger.removeClass("on");
        }
        break;
      case "sync":
        const sync = event.icon.find("[mod-port-symbol=sync]");
        const pulse = event.icon.find("[mod-port-symbol=pulse]");
        const tempo_factor = event.icon.find("[mod-port-symbol=tempo_factor]");

        if(value == 1) {
          sync.addClass("on");
          pulse.addClass("hide");
          tempo_factor.removeClass("hide");
        } else {
          sync.removeClass("on");
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