import rx
from rx import operators as ops
from rx.subject import Subject


class Storage:

    def __init__(self, init_msg):
        self.msg = init_msg
        self.observable = Subject()

    def start_link(self):
        def on_next(msg):
            self.msg = msg
        self.observable.subscribe(on_next)

    def set_msg(self, msg):
        self.observable.on_next(msg)

    def get_msg(self):
        return self.msg

