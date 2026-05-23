insert into UsrScope(usr, scope)
select 
	Usr.id, Scope.id
from 
	Usr, Scope
where
	Usr.name = $1 and Scope.name = $2
