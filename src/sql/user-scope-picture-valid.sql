select	
	count(1) = 1
from
	(select
		Scope.id
	from
		((select 
			UsrScope.usr, UsrScope.scope
		 from
			Usr join UsrScope on Usr.id = UsrScope.usr
		 where
			Usr.name = $1
		) as US join Scope on US.scope = Scope.id)
	 where 
	 	Scope.name = $2) as S
	join Picture on Picture.scope = S.id where Picture.id = $3
