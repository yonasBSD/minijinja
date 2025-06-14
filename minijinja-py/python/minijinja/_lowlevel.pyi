from typing import Any, Optional
from minijinja import Environment

class State:
    """A reference to the current state."""

    @property
    def env(self) -> Environment: ...
    @property
    def name(self) -> str: ...
    @property
    def auto_escape(self) -> Optional[str]: ...
    @property
    def current_block(self) -> Optional[str]: ...
    def lookup(self, name: str) -> Any: ...
    def get_temp(self, name: str, default: Optional[Any] = None) -> Any: ...
    def set_temp(self, name: str, value: Any) -> Any: ...
