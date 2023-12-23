class Stock:
    def __init__(self,
                 name: str,
                 open_price: float,
                 stop_loss: float = 0.0,
                 take_profit: float = 0.0) -> None:
        self.name: str = name
        self.open_price: str = open_price
        self.stop_loss: str = stop_loss
        self.take_profit: str = take_profit
        self.current_price: str = open_price

        def update_price(self, new_price: float) -> None:
            self.current_price = new_price
