<h1>
Internet del Cafe

<a href="../README.md">
  <img align="right" height="40"
  alt="EN" src="https://cdn-icons-png.flaticon.com/512/197/197484.png">
</a>

</h1>

![Maquina de Cafe](/assets/coffee_maker.gif)

Simule una maquina de cafe con multiples boquillas.

La máquina posee una cantidad predeterminada de **café**, **espuma**, **granos de café** (para moler), **leche** (para batir), así como un suministro ilimitado de **agua**.

Los pedidos se leen de `assets/orders.csv` como líneas que especifican _café (mg)_, _agua (ml)_ y _leche (ml)_, en ese orden, separados por comas.

## Diseño

La siguiente sección describe y justifica las decisiones de diseño tomadas para este proyecto.

### Ordenes

`orders` `take_orders`

Para emular [cómo funciona una máquina de café real](https://www.youtube.com/watch?v=ce22H2-0xh4&ab_channel=IntoTheOrdinary) de la manera más precisa posible; un dispensador debe aplicar primero el café, luego el agua y finalmente agregar la espuma

Se supone que las tazas no pueden contener más café o espuma que los recipientes de las máquinas. Lo que significa que cumplir con un pedido no requeriría múltiples moliendas de granos o batidos de leche.

También se supone que los recursos de la máquina son suficientes para cumplir con todos los pedidos.

Los pedidos se ponen encolan en un `Mutex<VecDeque>` y se los sigue utilizando un `Semaphore`.

> `Orders` proporciona una interfaz similar a una cola que puede ser usada de forma segura por varios hilos.
> Diferentemente, `pop` se bloqueará hasta que haya una orden disponible; que se logra mediante el uso de un 'Semáforo'.
> Esto significa que la estructura debe usarse a conciencia, ya que podría provocar un bloqueo permanente si no hay productores.

Un único subproceso se dedica a leer el archivo de entrada, actuando como único productor para los dispensadores que consumen pedidos.
Se utiliza un objeto `Orders::NoMoreOrders` para señalar que los dispensadores deben detenerse, después de cumplir con todos los pedidos.

### Recursos

`resources` `resource_monitor`

`Resources` proporciona una interfaz simple para usar una cantidad determinada de **café**, **agua** y **espuma**; asegurarse de que el café y la espuma sean utilizados por un solo consumidor a la vez; así como moler y batir la leche hasta llegar a la cantidad requerida.

El uso de todos los recursos se emula como un `sleep`, que depende linealmente de la cantidad de recursos utilizados.

Café/Granos y Espuma/Leche funcionan de manera análoga.

Los pedidos se cumplen de manera codiciosa; si un recurso necesita ser _transformado_, solo se procesará el mínimo requerido.

> Cuando se solicita una cantidad de café superior a la del recipiente, la máquina molerá los granos justos para cubrir la demanda.
>
> Cuando se solicita una cantidad de espuma superior a la del recipiente, la máquina batirá la leche suficiente para cubrir la demanda.

Esta última decisión podría resultar ineficiente si el tiempo fijo de procesamiento fuera elevado. Sin embargo, minimiza el desperdicio y hace los pedidos _individualmente_ más rápidos. Además, es un enfoque más realista, donde los ingredientes se mantienen lo más frescos posible.

Para poder monitorear los recursos incluso cuando se están utilizando, se utiliza un `ResourceMonitor` para realizar un seguimiento de la cantidad actual de recursos en un conjunto duplicado de campos que se actualiza después de su uso.

Se proporciona una interfaz de usuario amigable para monitorear los recursos en tiempo real.
Los datos se actualizan a intervalos, marcando recursos bajos adecuadamente.

```
Coffee: 100 mg
Coffee Beans: 8180 mg
Foam: 100 ml
Milk: 150 ml [WARNING: below threshold]
```

### Dispensadores

`dispenser`

Se proporciona una interfaz simple para consumir pedidos y usar recursos para cumplirlos.
Su implementación es trivial ya que la mayoría de la lógica es manejada por `Resources`

Un aspecto interesante de su funcionamiento es que cuando saca `NoMoreOrders` de la cola, lo vuelve a colocar y se detiene; para que otros dispensadores también puedan detenerse.

Si un pedido no se cumple, se ignorará, ya que se supone que los recursos de la máquina son suficientes para cumplir con todos los pedidos.

## Desarrollo

Este proyecto se inició con [Cargo](https://doc.rust-lang.org/cargo/), el administrador de paquetes de [Rust](https://www.rust-lang.org/).

Algunos comandos que puede encontrar útiles:

- `cargo test` - Ejecutar las pruebas.
- `cargo build` - Compila el proyecto.
- `cargo run` - Compila y ejecuta el proyecto.
