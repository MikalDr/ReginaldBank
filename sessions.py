from __future__ import annotations
from datetime import datetime
from typing import Optional

class Session:
    def __init__(self, name: Optional[str] = None, tag: Optional[str] = None, date: Optional[datetime] = None):
        if tag is None:
            self.tag = ""
        if date is None:
            self.date = datetime.now()
        if name is None:
            self.name = self.date.strftime("%y.%m.%d")
            
            
            

class SessionManager:
    def __init__(self):
        self.sessions: list[Session] = []
    
    def get_session_by_tag(self, tag: str) -> Optional[Session]:
        for session in self.sessions:
            if session.tag == tag:
                return session

    def get_sessions_by_date(self, date: datetime) -> list[Session]:
        day_before = date
        day_before.day -= 1
        day_before.time().hour = 0
        
        day_after = date
        day_after.day += 1
        day_after.time().hour = 0
        return [s for s in self.sessions if s.date == date or day_before < s.date < day_after]
    
    def add_session(self, session: Session) -> bool:
        b = session in self.sessions
        self.sessions.append(session)
        return b

    def get_session_by_date(self, date: datetime) -> Optional[Session]:
        return self.get_sessions_by_date(date).sort(lambda s: s.date, reverse=True)[0]

    