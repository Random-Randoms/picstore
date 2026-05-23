select
	Scope.id
from 
	(select scope from Usr left join UsrScope on Usr.id = UsrScope.usr where Usr.name = $1) as ScopeId
	left join Scope
	on ScopeId.scope = Scope.id
where 
	Scope.name = $2
